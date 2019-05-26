extern crate derive_attributes;
use derive_attributes::*;
#[derive(HelperAttr)]
struct Struct {
    #[helper] field: ()
}
fn main() {
    println!("Hello, world!");
}
