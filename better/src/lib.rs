extern crate proc_macro;
use std::ops::Index;

use proc_macro::{TokenStream, Ident};
use quote::*;
use syn::{parse_macro_input, ItemImpl, ImplItem, Type::{Path, self}, ReturnType};

#[proc_macro_attribute]
pub fn new(_: TokenStream, item: TokenStream) -> TokenStream {
    
    let input = parse_macro_input!(item as ItemImpl);
    let items: &ImplItem = input.items.index(0);
    
    let mut attr = quote!();
    let mut content = quote!();
    
    let name = if let Path(tp) = &*input.self_ty{
        Ok(tp.path.segments.last().unwrap().ident.clone())
    }else{ Err(()) };

    let name = name.unwrap();

    if let ImplItem::Fn(d) = items{
        let c = &d.block.stmts;
        let a = &d.attrs;

        content = quote!(#( #c )*);
        attr = quote!(#( #a, )*)
    }
    println!("\naaaaaaaaaaaa: {}\n", attr);
    let f = quote!(
        #[allow(non_snake_case)]
        fn #name () -> #name {
            #content
        }
    );
    TokenStream::from(f)
}