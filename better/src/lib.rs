extern crate proc_macro;
use proc_macro::TokenStream;
use quote::*;
use regex::Regex;
use syn::{parse_macro_input, ItemImpl};
#[proc_macro_attribute]
pub fn new(_: TokenStream, item: TokenStream) -> TokenStream {
    let item_string = item.to_string();
    
    let input = parse_macro_input!(item as ItemImpl);
    let name = format_ident!("{}", item_string.split(" ").nth(1).unwrap());
    // impl Asdf { fn new(a : isize, b : isize) -> Self { Self { a : a, b } } }
    // let reg = Regex::new(&format!("fn new\\((?<a>.*)\\) -> (Self)?({name})?")).unwrap();
    // let attr = &reg.captures(&item_string).unwrap()["a"];
    // let code = &reg.captures(&item_string).unwrap()["c"];
    
    // dbg!(match input.items.get(0).unwrap() {
    //     _ => {},
    // });
    println!("name: {name:?}");
    let f = quote!(
        fn #name() -> &'static str {
            "hello world"
        }
    );
    TokenStream::from(f)
}