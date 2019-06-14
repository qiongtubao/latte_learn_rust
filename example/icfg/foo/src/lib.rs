

#[cfg(feature = "foo")]
pub fn foo() {
    println!("hello1");
}

#[cfg(not(feature = "foo"))]
pub fn foo() {
    println!("hello2");
}
