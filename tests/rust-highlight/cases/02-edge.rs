union U { a: u32, b: f32 }
let p = &raw const x;
let q = &raw mut y;
default fn f() {}
async fn go() { x.await; }
fn lt<'a, 'b: 'a>(x: &'a str) -> &'b str { 'outer: loop { break 'outer; } }
let c1 = '\'';
let c2 = '\x41';
let c3 = '\u{1F600}';
let c4 = 'é';
let n1 = 0xffu8 + 1_000i64 + 2.5e10f64 + 0b101 + 0o7;
let ml = "multi
line string {} here";
let r2 = r##"has "# inside"##;
write!(f, "{} {}", a, b)?;
writeln!(f, "x{}", 1);
format!("{x}");
eprintln!(
    "err {}", e
);
macro_rules! m { ($x:expr) => { $x }; }
a::<T>::b();
Foo::bar();
foo::<u8>();
let Some(x) = opt else { return };
struct Gen<T: Clone + 'static> where T: Send { f: Box<dyn Fn(u8) -> u8> }
// unsafe in comment, "str" and 'a
let s = "unsafe struct // not comment";
/* unterminated-ish */ let z = 1;
pub(crate) const MAX: usize = 10;
static mut COUNTER: u32 = 0;
#[cfg(test)]
mod tests { #[test] fn t() { assert_eq!(1, 1); } }
let x: Result<(), Box<dyn Error>> = Ok(());
