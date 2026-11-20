//! `scmfmt [--check] FILE...`: Scheme のソースを整形する (`rseditor_core::scmfmt`).

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out = String::new();
    let code = rseditor_core::scmfmt::run(&args, &mut out);
    print!("{out}");
    std::process::exit(code);
}

#[cfg(tarpaulin_include)]
fn main() {}
