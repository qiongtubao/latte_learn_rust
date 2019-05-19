use  derive_set::New;
#[derive(New, PartialEq, Debug)]
pub struct Bar {
    pub x: i32,
    pub y: String,
}
fn main() {
let x = Bar::set(42, "hello".to_owned());
assert_eq!(x, Bar{x: 42, y: "hello".to_owned()});
}
