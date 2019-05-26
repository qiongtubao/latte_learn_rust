extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    println!("hello {:?}", _item.to_string());
    TokenStream::new()
}