// line comment
/// doc comment
//! inner doc
/* block /* nested */ still */
/** doc block */
#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Point<T> {
    x: T,
    pub y: i32,
}

enum Shape { Circle(f64), Square { side: u8 } }

unsafe fn danger<'a>(p: &'a mut Point<u8>) -> Option<&'a str> {
    let mut count = 0u8;
    let ref r = p;
    for item in 0..10 {
        println!("value {} and {:?}", item, r);
    }
    let s = "str \" esc";
    let c = 'a';
    let n = '\n';
    let raw = r#"raw "quoted""#;
    foo::bar::baz(1.5f32);
    std::mem::swap(&mut a, &mut b);
    let v = Vec::<u8>::new();
    let q = thing()?;
    vec![1, 2, 3];
    None
}

mod inner { use std::io; }
trait Tr { type Out; fn run(&self); }
impl<'a> Tr for Point<i32> { type Out = (); fn run(&self) {} }
