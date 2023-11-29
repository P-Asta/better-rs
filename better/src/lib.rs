extern crate proc_macro;
use std::ops::Index;

use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, ItemImpl, ImplItem, Type::Path};

#[proc_macro_attribute]
pub fn new(_: TokenStream, item: TokenStream) -> TokenStream {
    
    let input = parse_macro_input!(item as ItemImpl);
    let items: &ImplItem = input.items.index(0);
    
    let mut attr = quote!();
    let mut content = quote!();
    let name;

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

trait Calc{
    type Item;
    fn sum(&self) -> Self::Item;
    fn mul(&self) -> Self::Item;
}

macro_rules! calc_maker {
    ($($t: tt) *) => {
        $(
            impl Calc for Vec<$t>{
                type Item = $t;
                fn sum(&self) -> $t {
                    let mut res = 0;
                    for i in self.clone(){
                        res += i
                    }
                    res
                }
                fn mul(&self) -> $t {
                    let mut res = 0;
                    for i in self.clone(){
                        res *= i
                    }
                    res
                }
            }
        )*
    };
}
calc_maker!{
    isize
    i8
    i16
    i32
    i64
    i128

    usize
    u8
    u16
    u32
    u64
    u128
}