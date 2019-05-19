extern crate derive_attributes;

use derive_attributes::show_streams;

// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

// Example: Attribute has a metaitem
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

// Example: Multiple words in metaitem
#[show_streams(multiple words)]
fn invoke3() {}
// out: attr: "multiple words"
// out: item: "fn invoke3() {}"

// Example:
#[show_streams { delimiters }]
fn invoke4() {}
fn main() {
    println!("hello");
}