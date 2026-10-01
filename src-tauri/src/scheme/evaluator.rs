use crate::scheme::parser::Parser;
use crate::scheme::value::{new_env, BuiltinFn, Env, Lambda, Value};
use std::sync::Arc;

const MAX_DEPTH: usize = 10_000;
const STACK_SIZE: usize = 256 * 1024 * 1024;

/// 評価中の状態 (標準出力バッファと再帰の深さ).
pub struct Interp {
    pub out: String,
    depth: usize,
}

pub struct Evaluator {
    global: Env,
}

impl Evaluator {
    pub fn new() -> Self {
        let global = new_env(None);
        register_builtins(&global);
        Evaluator { global }
    }

    pub fn eval(&self, code: &str) -> Result<String, String> {
        let mut parser = Parser::new(code);
        let values = parser.parse()?;

        if values.is_empty() {
            return Ok(String::new());
        }

        let global = self.global.clone();
        // 深い再帰に耐えるため, 大きなスタックを持つスレッドで評価する.
        std::thread::scope(|scope| {
            std::thread::Builder::new()
                .stack_size(STACK_SIZE)
                .spawn_scoped(scope, move || run(values, global))
                .map_err(|e| e.to_string())?
                .join()
                .map_err(|_| "Internal evaluator error".to_string())?
        })
    }
}

fn run(values: Vec<Value>, global: Env) -> Result<String, String> {
    let mut interp = Interp {
        out: String::new(),
        depth: 0,
    };
    let mut results = Vec::new();
    for value in values {
        let result = interp.eval(value, global.clone());
        if !interp.out.is_empty() {
            results.push(std::mem::take(&mut interp.out));
        }
        let result = result?;
        if !matches!(result, Value::Void) {
            results.push(result.to_string());
        }
    }
    Ok(results.join("\n"))
}

fn env_get(env: &Env, name: &str) -> Option<Value> {
    let mut cur = env.clone();
    loop {
        let parent = {
            let frame = cur.lock().unwrap();
            if let Some(v) = frame.vars.get(name) {
                return Some(v.clone());
            }
            frame.parent.clone()
        };
        cur = parent?;
    }
}

fn env_set(env: &Env, name: &str, value: Value) -> Result<(), String> {
    let mut cur = env.clone();
    loop {
        let parent = {
            let mut frame = cur.lock().unwrap();
            if let Some(slot) = frame.vars.get_mut(name) {
                *slot = value;
                return Ok(());
            }
            frame.parent.clone()
        };
        cur = parent.ok_or_else(|| format!("Unbound variable: {}", name))?;
    }
}

fn env_define(env: &Env, name: &str, value: Value) {
    env.lock().unwrap().vars.insert(name.to_string(), value);
}

fn symbol_name(v: &Value) -> Result<String, String> {
    match v {
        Value::Symbol(s) => Ok(s.clone()),
        other => Err(format!("Expected a symbol, got {}", other)),
    }
}

/// 仮引数リストを (固定引数, 残余引数) に分解する.
fn parse_params(params: &Value) -> Result<(Vec<String>, Option<String>), String> {
    match params {
        Value::Symbol(s) => Ok((vec![], Some(s.clone()))),
        Value::List(items) => {
            let mut names = Vec::new();
            let mut rest = None;
            let mut i = 0;
            while i < items.len() {
                let name = symbol_name(&items[i])?;
                if name == "." {
                    if i + 2 != items.len() {
                        return Err("Invalid parameter list".to_string());
                    }
                    rest = Some(symbol_name(&items[i + 1])?);
                    break;
                }
                names.push(name);
                i += 1;
            }
            Ok((names, rest))
        }
        _ => Err("Invalid parameter list".to_string()),
    }
}

fn make_lambda(
    name: Option<String>,
    params: &Value,
    body: &[Value],
    env: &Env,
) -> Result<Value, String> {
    if body.is_empty() {
        return Err("lambda requires a body".to_string());
    }
    let (params, rest) = parse_params(params)?;
    Ok(Value::Lambda(Arc::new(Lambda {
        name,
        params,
        rest,
        body: body.to_vec(),
        env: env.clone(),
    })))
}

impl Interp {
    pub fn eval(&mut self, expr: Value, env: Env) -> Result<Value, String> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            self.depth -= 1;
            return Err("Recursion depth exceeded".to_string());
        }
        let result = self.eval_inner(expr, env);
        self.depth -= 1;
        result
    }

    /// body の最後以外を評価し, 末尾式を返す (末尾呼び出し最適化用).
    fn eval_body_head(&mut self, body: &[Value], env: &Env) -> Result<Value, String> {
        let (last, init) = body.split_last().ok_or_else(|| "Empty body".to_string())?;
        for e in init {
            self.eval(e.clone(), env.clone())?;
        }
        Ok(last.clone())
    }

    fn eval_inner(&mut self, mut expr: Value, mut env: Env) -> Result<Value, String> {
        loop {
            let items = match &expr {
                Value::Symbol(name) => {
                    return env_get(&env, name)
                        .ok_or_else(|| format!("Unbound variable: {}", name));
                }
                Value::List(items) if !items.is_empty() => items.clone(),
                other => return Ok(other.clone()),
            };
            let args = &items[1..];

            if let Value::Symbol(op) = &items[0] {
                match op.as_str() {
                    "quote" => {
                        if args.len() != 1 {
                            return Err("quote expects 1 argument".to_string());
                        }
                        return Ok(args[0].clone());
                    }
                    "if" => {
                        if args.len() < 2 || args.len() > 3 {
                            return Err("if expects 2 or 3 arguments".to_string());
                        }
                        let cond = self.eval(args[0].clone(), env.clone())?;
                        if cond.is_true() {
                            expr = args[1].clone();
                        } else if args.len() == 3 {
                            expr = args[2].clone();
                        } else {
                            return Ok(Value::Void);
                        }
                        continue;
                    }
                    "define" => {
                        if args.len() < 2 {
                            return Err("define expects a name and a value".to_string());
                        }
                        match &args[0] {
                            Value::Symbol(name) => {
                                if args.len() != 2 {
                                    return Err("define expects 2 arguments".to_string());
                                }
                                let mut v = self.eval(args[1].clone(), env.clone())?;
                                if let Value::Lambda(l) = &v {
                                    if l.name.is_none() {
                                        v = Value::Lambda(Arc::new(Lambda {
                                            name: Some(name.clone()),
                                            params: l.params.clone(),
                                            rest: l.rest.clone(),
                                            body: l.body.clone(),
                                            env: l.env.clone(),
                                        }));
                                    }
                                }
                                env_define(&env, name, v);
                                return Ok(Value::Void);
                            }
                            Value::List(sig) if !sig.is_empty() => {
                                let name = symbol_name(&sig[0])?;
                                let params = Value::List(sig[1..].to_vec());
                                let f = make_lambda(Some(name.clone()), &params, &args[1..], &env)?;
                                env_define(&env, &name, f);
                                return Ok(Value::Void);
                            }
                            _ => return Err("Invalid define".to_string()),
                        }
                    }
                    "set!" => {
                        if args.len() != 2 {
                            return Err("set! expects 2 arguments".to_string());
                        }
                        let name = symbol_name(&args[0])?;
                        let v = self.eval(args[1].clone(), env.clone())?;
                        env_set(&env, &name, v)?;
                        return Ok(Value::Void);
                    }
                    "lambda" => {
                        if args.len() < 2 {
                            return Err("lambda expects parameters and a body".to_string());
                        }
                        return make_lambda(None, &args[0], &args[1..], &env);
                    }
                    "begin" => {
                        if args.is_empty() {
                            return Ok(Value::Void);
                        }
                        expr = self.eval_body_head(args, &env)?;
                        continue;
                    }
                    "let" | "let*" | "letrec" => {
                        if args.len() < 2 {
                            return Err(format!("{} expects bindings and a body", op));
                        }
                        // 名前付き let
                        if op == "let" {
                            if let Value::Symbol(loop_name) = &args[0] {
                                if args.len() < 3 {
                                    return Err("named let expects bindings and a body".to_string());
                                }
                                let (names, inits) = parse_bindings(&args[1])?;
                                let mut vals = Vec::new();
                                for init in inits {
                                    vals.push(self.eval(init, env.clone())?);
                                }
                                let loop_env = new_env(Some(env.clone()));
                                let params = Value::List(
                                    names.iter().map(|n| Value::Symbol(n.clone())).collect(),
                                );
                                let f = make_lambda(
                                    Some(loop_name.clone()),
                                    &params,
                                    &args[2..],
                                    &loop_env,
                                )?;
                                env_define(&loop_env, loop_name, f.clone());
                                let Value::Lambda(l) = f else { unreachable!() };
                                let (body_env, tail) = self.bind_call(&l, vals)?;
                                env = body_env;
                                expr = tail;
                                continue;
                            }
                        }
                        let (names, inits) = parse_bindings(&args[0])?;
                        let new = new_env(Some(env.clone()));
                        match op.as_str() {
                            "let" => {
                                let mut vals = Vec::new();
                                for init in inits {
                                    vals.push(self.eval(init, env.clone())?);
                                }
                                for (n, v) in names.iter().zip(vals) {
                                    env_define(&new, n, v);
                                }
                            }
                            "let*" => {
                                let mut cur = env.clone();
                                for (n, init) in names.iter().zip(inits) {
                                    let v = self.eval(init, cur.clone())?;
                                    cur = new_env(Some(cur));
                                    env_define(&cur, n, v);
                                }
                                env = cur;
                                expr = self.eval_body_head(&args[1..], &env)?;
                                continue;
                            }
                            _ => {
                                for (n, init) in names.iter().zip(inits) {
                                    let v = self.eval(init, new.clone())?;
                                    env_define(&new, n, v);
                                }
                            }
                        }
                        env = new;
                        expr = self.eval_body_head(&args[1..], &env)?;
                        continue;
                    }
                    "cond" => {
                        let mut next = None;
                        for clause in args {
                            let Value::List(parts) = clause else {
                                return Err("Invalid cond clause".to_string());
                            };
                            if parts.is_empty() {
                                return Err("Invalid cond clause".to_string());
                            }
                            let is_else = parts[0] == Value::Symbol("else".to_string());
                            let test = if is_else {
                                Value::Boolean(true)
                            } else {
                                self.eval(parts[0].clone(), env.clone())?
                            };
                            if test.is_true() {
                                if parts.len() == 1 {
                                    return Ok(test);
                                }
                                next = Some(self.eval_body_head(&parts[1..], &env)?);
                                break;
                            }
                        }
                        match next {
                            Some(e) => {
                                expr = e;
                                continue;
                            }
                            None => return Ok(Value::Void),
                        }
                    }
                    "case" => {
                        if args.is_empty() {
                            return Err("case expects a key".to_string());
                        }
                        let key = self.eval(args[0].clone(), env.clone())?;
                        let mut next = None;
                        for clause in &args[1..] {
                            let Value::List(parts) = clause else {
                                return Err("Invalid case clause".to_string());
                            };
                            if parts.len() < 2 {
                                return Err("Invalid case clause".to_string());
                            }
                            let matched = match &parts[0] {
                                Value::Symbol(s) if s == "else" => true,
                                Value::List(data) => data.contains(&key),
                                _ => return Err("Invalid case clause".to_string()),
                            };
                            if matched {
                                next = Some(self.eval_body_head(&parts[1..], &env)?);
                                break;
                            }
                        }
                        match next {
                            Some(e) => {
                                expr = e;
                                continue;
                            }
                            None => return Ok(Value::Void),
                        }
                    }
                    "and" => {
                        if args.is_empty() {
                            return Ok(Value::Boolean(true));
                        }
                        let (last, init) = args.split_last().unwrap();
                        for e in init {
                            let v = self.eval(e.clone(), env.clone())?;
                            if !v.is_true() {
                                return Ok(v);
                            }
                        }
                        expr = last.clone();
                        continue;
                    }
                    "or" => {
                        if args.is_empty() {
                            return Ok(Value::Boolean(false));
                        }
                        let (last, init) = args.split_last().unwrap();
                        for e in init {
                            let v = self.eval(e.clone(), env.clone())?;
                            if v.is_true() {
                                return Ok(v);
                            }
                        }
                        expr = last.clone();
                        continue;
                    }
                    "when" | "unless" => {
                        if args.len() < 2 {
                            return Err(format!("{} expects a test and a body", op));
                        }
                        let test = self.eval(args[0].clone(), env.clone())?.is_true();
                        if test == (op == "when") {
                            expr = self.eval_body_head(&args[1..], &env)?;
                            continue;
                        }
                        return Ok(Value::Void);
                    }
                    _ => {}
                }
            }

            // 手続き呼び出し
            let f = self.eval(items[0].clone(), env.clone())?;
            let mut vals = Vec::with_capacity(args.len());
            for a in args {
                vals.push(self.eval(a.clone(), env.clone())?);
            }
            match f {
                Value::Function(_, func) => return func(self, vals),
                Value::Lambda(l) => {
                    let (body_env, tail) = self.bind_call(&l, vals)?;
                    env = body_env;
                    expr = tail;
                }
                other => return Err(format!("Not a procedure: {}", other)),
            }
        }
    }

    /// 呼び出し用の環境を作り, 本体の末尾式を返す.
    fn bind_call(&mut self, l: &Lambda, vals: Vec<Value>) -> Result<(Env, Value), String> {
        let name = l.name.as_deref().unwrap_or("lambda");
        if vals.len() < l.params.len() || (l.rest.is_none() && vals.len() > l.params.len()) {
            return Err(format!(
                "{} expects {}{} argument(s), got {}",
                name,
                if l.rest.is_some() { "at least " } else { "" },
                l.params.len(),
                vals.len()
            ));
        }
        let call_env = new_env(Some(l.env.clone()));
        let mut vals = vals.into_iter();
        for p in &l.params {
            env_define(&call_env, p, vals.next().unwrap());
        }
        if let Some(r) = &l.rest {
            env_define(&call_env, r, Value::List(vals.collect()));
        }
        let tail = self.eval_body_head(&l.body, &call_env)?;
        Ok((call_env, tail))
    }

    /// 手続き値を引数に適用する (組み込み関数から使用).
    pub fn apply(&mut self, f: &Value, vals: Vec<Value>) -> Result<Value, String> {
        match f {
            Value::Function(_, func) => func(self, vals),
            Value::Lambda(l) => {
                let (env, tail) = self.bind_call(l, vals)?;
                self.eval(tail, env)
            }
            other => Err(format!("Not a procedure: {}", other)),
        }
    }
}

fn parse_bindings(bindings: &Value) -> Result<(Vec<String>, Vec<Value>), String> {
    let Value::List(list) = bindings else {
        return Err("Invalid bindings".to_string());
    };
    let mut names = Vec::new();
    let mut inits = Vec::new();
    for b in list {
        match b {
            Value::List(pair) if pair.len() == 2 => {
                names.push(symbol_name(&pair[0])?);
                inits.push(pair[1].clone());
            }
            _ => return Err("Invalid binding".to_string()),
        }
    }
    Ok((names, inits))
}

// ---- 組み込み関数 ----

fn num(name: &str, v: &Value) -> Result<f64, String> {
    match v {
        Value::Number(n) => Ok(*n),
        other => Err(format!("{} expects numbers, got {}", name, other)),
    }
}

fn nums(name: &str, args: &[Value]) -> Result<Vec<f64>, String> {
    args.iter().map(|a| num(name, a)).collect()
}

fn arity(name: &str, args: &[Value], n: usize) -> Result<(), String> {
    if args.len() == n {
        Ok(())
    } else {
        Err(format!(
            "{} expects {} argument(s), got {}",
            name,
            n,
            args.len()
        ))
    }
}

fn list_arg<'a>(name: &str, v: &'a Value) -> Result<&'a Vec<Value>, String> {
    match v {
        Value::List(items) => Ok(items),
        other => Err(format!("{} expects a list, got {}", name, other)),
    }
}

fn string_arg<'a>(name: &str, v: &'a Value) -> Result<&'a str, String> {
    match v {
        Value::String(s) => Ok(s),
        other => Err(format!("{} expects a string, got {}", name, other)),
    }
}

fn compare(name: &str, args: &[Value], cmp: fn(f64, f64) -> bool) -> Result<Value, String> {
    let ns = nums(name, args)?;
    Ok(Value::Boolean(ns.windows(2).all(|w| cmp(w[0], w[1]))))
}

fn register_builtins(env: &Env) {
    let mut reg = |name: &str, f: BuiltinFn| {
        env_define(env, name, Value::Function(name.to_string(), f));
    };

    reg("+", |_, a| Ok(Value::Number(nums("+", &a)?.iter().sum())));
    reg("*", |_, a| {
        Ok(Value::Number(nums("*", &a)?.iter().product()))
    });
    reg("-", |_, a| {
        let ns = nums("-", &a)?;
        match ns.split_first() {
            None => Err("- expects at least one argument".to_string()),
            Some((x, [])) => Ok(Value::Number(-x)),
            Some((x, rest)) => Ok(Value::Number(rest.iter().fold(*x, |acc, n| acc - n))),
        }
    });
    reg("/", |_, a| {
        let ns = nums("/", &a)?;
        let (first, rest) = ns
            .split_first()
            .ok_or_else(|| "/ expects at least one argument".to_string())?;
        let (mut acc, divisors) = if rest.is_empty() {
            (1.0, &ns[..])
        } else {
            (*first, rest)
        };
        for d in divisors {
            if *d == 0.0 {
                return Err("Division by zero".to_string());
            }
            acc /= d;
        }
        Ok(Value::Number(acc))
    });
    reg("=", |_, a| compare("=", &a, |x, y| x == y));
    reg("<", |_, a| compare("<", &a, |x, y| x < y));
    reg(">", |_, a| compare(">", &a, |x, y| x > y));
    reg("<=", |_, a| compare("<=", &a, |x, y| x <= y));
    reg(">=", |_, a| compare(">=", &a, |x, y| x >= y));

    reg("quotient", |_, a| int_op("quotient", &a, |x, y| x / y));
    reg("remainder", |_, a| int_op("remainder", &a, |x, y| x % y));
    reg("modulo", |_, a| {
        int_op("modulo", &a, |x, y| {
            x.rem_euclid(y) + if y < 0 && x.rem_euclid(y) != 0 { y } else { 0 }
        })
    });
    reg("abs", |_, a| {
        arity("abs", &a, 1)?;
        Ok(Value::Number(num("abs", &a[0])?.abs()))
    });
    reg("min", |_, a| {
        let ns = nums("min", &a)?;
        ns.into_iter()
            .reduce(f64::min)
            .map(Value::Number)
            .ok_or_else(|| "min expects at least one argument".to_string())
    });
    reg("max", |_, a| {
        let ns = nums("max", &a)?;
        ns.into_iter()
            .reduce(f64::max)
            .map(Value::Number)
            .ok_or_else(|| "max expects at least one argument".to_string())
    });
    reg("zero?", |_, a| {
        arity("zero?", &a, 1)?;
        Ok(Value::Boolean(num("zero?", &a[0])? == 0.0))
    });
    reg("even?", |_, a| {
        arity("even?", &a, 1)?;
        Ok(Value::Boolean(num("even?", &a[0])? % 2.0 == 0.0))
    });
    reg("odd?", |_, a| {
        arity("odd?", &a, 1)?;
        Ok(Value::Boolean(num("odd?", &a[0])?.abs() % 2.0 == 1.0))
    });

    reg("not", |_, a| {
        arity("not", &a, 1)?;
        Ok(Value::Boolean(!a[0].is_true()))
    });
    for name in ["eq?", "eqv?", "equal?"] {
        reg(name, |_, a| {
            arity("equal?", &a, 2)?;
            Ok(Value::Boolean(a[0] == a[1]))
        });
    }

    // 型述語
    reg("number?", |_, a| {
        pred("number?", &a, |v| matches!(v, Value::Number(_)))
    });
    reg("integer?", |_, a| {
        pred(
            "integer?",
            &a,
            |v| matches!(v, Value::Number(n) if n.fract() == 0.0),
        )
    });
    reg("string?", |_, a| {
        pred("string?", &a, |v| matches!(v, Value::String(_)))
    });
    reg("symbol?", |_, a| {
        pred("symbol?", &a, |v| matches!(v, Value::Symbol(_)))
    });
    reg("boolean?", |_, a| {
        pred("boolean?", &a, |v| matches!(v, Value::Boolean(_)))
    });
    reg("list?", |_, a| {
        pred("list?", &a, |v| matches!(v, Value::List(_)))
    });
    reg("pair?", |_, a| {
        pred(
            "pair?",
            &a,
            |v| matches!(v, Value::List(l) if !l.is_empty()),
        )
    });
    reg("null?", |_, a| {
        pred("null?", &a, |v| matches!(v, Value::List(l) if l.is_empty()))
    });
    reg("procedure?", |_, a| {
        pred("procedure?", &a, |v| {
            matches!(v, Value::Function(..) | Value::Lambda(_))
        })
    });

    // リスト操作
    reg("list", |_, a| Ok(Value::List(a)));
    reg("car", |_, a| {
        arity("car", &a, 1)?;
        list_arg("car", &a[0])?
            .first()
            .cloned()
            .ok_or_else(|| "car: empty list".to_string())
    });
    reg("cdr", |_, a| {
        arity("cdr", &a, 1)?;
        let l = list_arg("cdr", &a[0])?;
        if l.is_empty() {
            return Err("cdr: empty list".to_string());
        }
        Ok(Value::List(l[1..].to_vec()))
    });
    reg("cons", |_, a| {
        arity("cons", &a, 2)?;
        let tail = list_arg("cons", &a[1])?;
        let mut out = vec![a[0].clone()];
        out.extend(tail.iter().cloned());
        Ok(Value::List(out))
    });
    reg("append", |_, a| {
        let mut out = Vec::new();
        for l in &a {
            out.extend(list_arg("append", l)?.iter().cloned());
        }
        Ok(Value::List(out))
    });
    reg("length", |_, a| {
        arity("length", &a, 1)?;
        Ok(Value::Number(list_arg("length", &a[0])?.len() as f64))
    });
    reg("reverse", |_, a| {
        arity("reverse", &a, 1)?;
        let mut l = list_arg("reverse", &a[0])?.clone();
        l.reverse();
        Ok(Value::List(l))
    });
    reg("list-ref", |_, a| {
        arity("list-ref", &a, 2)?;
        let l = list_arg("list-ref", &a[0])?;
        let i = num("list-ref", &a[1])?;
        if i < 0.0 || i.fract() != 0.0 {
            return Err("list-ref: invalid index".to_string());
        }
        l.get(i as usize)
            .cloned()
            .ok_or_else(|| "list-ref: index out of range".to_string())
    });
    reg("map", |it, a| {
        arity("map", &a, 2)?;
        let mut out = Vec::new();
        for x in list_arg("map", &a[1])? {
            out.push(it.apply(&a[0], vec![x.clone()])?);
        }
        Ok(Value::List(out))
    });
    reg("for-each", |it, a| {
        arity("for-each", &a, 2)?;
        for x in list_arg("for-each", &a[1])? {
            it.apply(&a[0], vec![x.clone()])?;
        }
        Ok(Value::Void)
    });
    reg("filter", |it, a| {
        arity("filter", &a, 2)?;
        let mut out = Vec::new();
        for x in list_arg("filter", &a[1])? {
            if it.apply(&a[0], vec![x.clone()])?.is_true() {
                out.push(x.clone());
            }
        }
        Ok(Value::List(out))
    });
    reg("fold", |it, a| {
        arity("fold", &a, 3)?;
        let mut acc = a[1].clone();
        for x in list_arg("fold", &a[2])? {
            acc = it.apply(&a[0], vec![x.clone(), acc])?;
        }
        Ok(acc)
    });
    reg("reduce", |it, a| {
        arity("reduce", &a, 3)?;
        let l = list_arg("reduce", &a[2])?;
        let Some((first, rest)) = l.split_first() else {
            return Ok(a[1].clone());
        };
        let mut acc = first.clone();
        for x in rest {
            acc = it.apply(&a[0], vec![x.clone(), acc])?;
        }
        Ok(acc)
    });
    reg("apply", |it, a| {
        arity("apply", &a, 2)?;
        let args = list_arg("apply", &a[1])?.clone();
        it.apply(&a[0], args)
    });

    // 文字列
    reg("string-append", |_, a| {
        let mut out = String::new();
        for s in &a {
            out.push_str(string_arg("string-append", s)?);
        }
        Ok(Value::String(out))
    });
    reg("string-length", |_, a| {
        arity("string-length", &a, 1)?;
        Ok(Value::Number(
            string_arg("string-length", &a[0])?.chars().count() as f64,
        ))
    });
    reg("number->string", |_, a| {
        arity("number->string", &a, 1)?;
        Ok(Value::String(
            Value::Number(num("number->string", &a[0])?).to_string(),
        ))
    });

    // 入出力
    reg("display", |it, a| {
        arity("display", &a, 1)?;
        it.out.push_str(&a[0].display_string());
        Ok(Value::Void)
    });
    reg("newline", |it, _| {
        it.out.push('\n');
        Ok(Value::Void)
    });
}

fn pred(name: &str, args: &[Value], f: fn(&Value) -> bool) -> Result<Value, String> {
    arity(name, args, 1)?;
    Ok(Value::Boolean(f(&args[0])))
}

fn int_op(name: &str, args: &[Value], f: fn(i64, i64) -> i64) -> Result<Value, String> {
    arity(name, args, 2)?;
    let (x, y) = (num(name, &args[0])?, num(name, &args[1])?);
    if x.fract() != 0.0 || y.fract() != 0.0 {
        return Err(format!("{} expects integers", name));
    }
    if y == 0.0 {
        return Err("Division by zero".to_string());
    }
    Ok(Value::Number(f(x as i64, y as i64) as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_addition() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(+ 1 2 3)").unwrap(), "6");
    }

    #[test]
    fn evaluates_subtraction() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(- 10 3)").unwrap(), "7");
    }

    #[test]
    fn evaluates_unary_minus_as_negation() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(- 5)").unwrap(), "-5");
    }

    #[test]
    fn evaluates_multiplication() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(* 2 3 4)").unwrap(), "24");
    }

    #[test]
    fn evaluates_division() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(/ 20 4)").unwrap(), "5");
    }

    #[test]
    fn division_by_zero_is_an_error() {
        let eval = Evaluator::new();
        assert!(eval.eval("(/ 1 0)").is_err());
    }

    #[test]
    fn evaluates_nested_expressions() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(+ 1 (* 2 3))").unwrap(), "7");
    }

    #[test]
    fn evaluates_quote() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(quote (a b c))").unwrap(), "(a b c)");
    }

    #[test]
    fn evaluates_list() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(list 1 2 3)").unwrap(), "(1 2 3)");
    }

    #[test]
    fn evaluates_multiple_top_level_forms_line_by_line() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("(+ 1 1)\n(+ 2 2)").unwrap(), "2\n4");
    }

    #[test]
    fn empty_input_evaluates_to_empty_string() {
        let eval = Evaluator::new();
        assert_eq!(eval.eval("").unwrap(), "");
    }

    #[test]
    fn unknown_operator_is_an_error() {
        let eval = Evaluator::new();
        assert!(eval.eval("(unknown-op 1 2)").is_err());
    }

    #[test]
    fn wrong_argument_type_is_an_error() {
        let eval = Evaluator::new();
        assert!(eval.eval(r#"(+ 1 "two")"#).is_err());
    }

    fn ev(code: &str) -> String {
        Evaluator::new().eval(code).unwrap()
    }

    #[test]
    fn evaluates_comparisons() {
        assert_eq!(ev("(< 1 2 3)"), "#t");
        assert_eq!(ev("(>= 3 3 4)"), "#f");
        assert_eq!(ev("(= 2 2)"), "#t");
    }

    #[test]
    fn evaluates_if_cond_case() {
        assert_eq!(ev("(if (> 2 1) 'yes 'no)"), "yes");
        assert_eq!(ev("(cond ((< 2 1) 1) ((= 2 2) 2) (else 3))"), "2");
        assert_eq!(
            ev("(case (* 2 3) ((2 3 5 7) 'prime) ((1 4 6 8 9) 'composite))"),
            "composite"
        );
    }

    #[test]
    fn define_and_lambda() {
        assert_eq!(ev("(define x 10)\n(+ x 5)"), "15");
        assert_eq!(ev("(define (sq n) (* n n))\n(sq 7)"), "49");
        assert_eq!(ev("((lambda (a b) (+ a b)) 1 2)"), "3");
        assert_eq!(ev("(define (f . xs) xs)\n(f 1 2 3)"), "(1 2 3)");
    }

    #[test]
    fn closures_capture_and_mutate() {
        let code = "(define (make-counter) (let ((n 0)) (lambda () (set! n (+ n 1)) n)))
(define c (make-counter))
(c)
(c)";
        assert_eq!(ev(code), "1\n2");
    }

    #[test]
    fn recursion_factorial() {
        assert_eq!(
            ev("(define (fact n) (if (= n 0) 1 (* n (fact (- n 1)))))\n(fact 10)"),
            "3628800"
        );
    }

    #[test]
    fn tail_calls_do_not_overflow() {
        let code =
            "(define (loop i acc) (if (= i 0) acc (loop (- i 1) (+ acc 1))))\n(loop 100000 0)";
        assert_eq!(ev(code), "100000");
    }

    #[test]
    fn infinite_recursion_is_an_error() {
        let code = "(define (f n) (+ 1 (f n)))\n(f 1)";
        assert!(Evaluator::new().eval(code).is_err());
    }

    #[test]
    fn let_forms() {
        assert_eq!(ev("(let ((a 1) (b 2)) (+ a b))"), "3");
        assert_eq!(ev("(let* ((a 1) (b (+ a 1))) (* a b))"), "2");
        assert_eq!(
            ev("(letrec ((ev? (lambda (n) (if (= n 0) #t (od? (- n 1))))) (od? (lambda (n) (if (= n 0) #f (ev? (- n 1)))))) (ev? 10))"),
            "#t"
        );
        assert_eq!(
            ev("(let loop ((i 0) (acc '())) (if (= i 3) acc (loop (+ i 1) (cons i acc))))"),
            "(2 1 0)"
        );
    }

    #[test]
    fn list_operations() {
        assert_eq!(ev("(car '(1 2 3))"), "1");
        assert_eq!(ev("(cdr '(1 2 3))"), "(2 3)");
        assert_eq!(ev("(cons 1 '(2 3))"), "(1 2 3)");
        assert_eq!(ev("(append '(1) '(2 3))"), "(1 2 3)");
        assert_eq!(ev("(length '(1 2 3))"), "3");
        assert_eq!(ev("(reverse '(1 2 3))"), "(3 2 1)");
        assert_eq!(ev("(null? '())"), "#t");
        assert!(Evaluator::new().eval("(car '())").is_err());
    }

    #[test]
    fn higher_order_functions() {
        assert_eq!(ev("(map (lambda (x) (* x x)) '(1 2 3))"), "(1 4 9)");
        assert_eq!(ev("(filter odd? '(1 2 3 4 5))"), "(1 3 5)");
        assert_eq!(ev("(fold + 0 '(1 2 3 4))"), "10");
        assert_eq!(ev("(apply + '(1 2 3))"), "6");
    }

    #[test]
    fn logic_forms() {
        assert_eq!(ev("(and 1 2 3)"), "3");
        assert_eq!(ev("(and 1 #f 3)"), "#f");
        assert_eq!(ev("(or #f #f 5)"), "5");
        assert_eq!(ev("(not #f)"), "#t");
    }

    #[test]
    fn strings_and_output() {
        assert_eq!(ev(r#"(string-append "foo" "bar")"#), "\"foobar\"");
        assert_eq!(ev(r#"(display "hi")"#), "hi");
        assert_eq!(ev("(modulo -7 3)"), "2");
        assert_eq!(ev("(quotient 7 2)"), "3");
    }

    #[test]
    fn errors_are_reported() {
        let e = Evaluator::new();
        assert!(e.eval("undefined-var").is_err());
        assert!(e.eval("((lambda (x) x))").is_err());
        assert!(e.eval("(1 2 3)").is_err());
    }

    #[test]
    fn definitions_persist_across_calls() {
        let e = Evaluator::new();
        e.eval("(define y 4)").unwrap();
        assert_eq!(e.eval("(* y y)").unwrap(), "16");
    }
}
