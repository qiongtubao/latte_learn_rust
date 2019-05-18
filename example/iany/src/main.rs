use std::any::{Any, TypeId};
#[derive(Debug)]
enum E {
    H, 
    He,
    Li
}
struct S {
    x: u8,
    y: u8,
    z: u16
}
fn main() {
    let v1 = 0xc0ffee_u32;
    let v2 = E::He;
    let v3 = S {x: 0xde, y: 0xad, z: 0xbeef};
    let v4 = "rust".to_string();
    let mut a: &Any;
    a = &v1;
    assert!(a.is::<u32>());
    println!("{:?}", TypeId::of::<u32>());
    println!("Hello, world!");
    a = &v4;
    if let Some(v) = a.downcast_ref::<String>() {
        println!("string {:?}", v);
    }
}
