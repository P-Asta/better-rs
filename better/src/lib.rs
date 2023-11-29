extern crate proc_macro;
use std::ops::Index;

use proc_macro::{TokenStream, Ident};
use quote::*;
use syn::{parse_macro_input, ItemImpl, ImplItem, Type::{Path, self}, ReturnType};

#[proc_macro_attribute]
pub fn new(asdf: TokenStream, item: TokenStream) -> TokenStream {
    
    let input = parse_macro_input!(item as ItemImpl);
    let items: &ImplItem = input.items.index(0);
    
    let mut attr = quote!();
    let mut content = quote!();
    let mut name;

    if let Path(tp) = &*input.self_ty{
        name = if let Some(o) = tp.path.segments.last(){
            format_ident!("{}", o.ident.clone().to_string())
        }else{ format_ident!("wtf") }
    }else { name = format_ident!("wtf") }
    


    if let ImplItem::Fn(d) = items{
        let c = &d.block.stmts;
        let a = &d.sig.inputs;

        content = quote!(#( #c )*);
        attr = quote!(#a)
        // attr = quote!(#( #a, )*)
    }
    
    let f = quote!(
        #input
        #[allow(non_snake_case)]
        fn #name (#attr) -> #name {
            #content
        }
    );
    TokenStream::from(f)
}