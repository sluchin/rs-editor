//! `scmlint FILE...`: Scheme のソースの誤りを見つける (`rseditor_core::scmlint`).

#[cfg(not(tarpaulin_include))]
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out = String::new();
    let code = rseditor_core::scmlint::run(&args, &mut out);
    print!("{out}");
    std::process::exit(code);
}

#[cfg(tarpaulin_include)]
fn main() {}
