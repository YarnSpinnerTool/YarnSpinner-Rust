//! Taken from <https://github.com/bevyengine/bevy/blob/fe852fd0adbce6856f5886d66d20d62cfc936287/crates/bevy_utils/macros/src/lib.rs>

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
    Ident, LitInt, Result,
};

struct AllTuples {
    macro_ident: Ident,
    start: usize,
    end: usize,
}

impl Parse for AllTuples {
    fn parse(input: ParseStream) -> Result<Self> {
        let macro_ident = input.parse::<Ident>()?;
        input.parse::<Comma>()?;
        let start = input.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Comma>()?;
        let end = input.parse::<LitInt>()?.base10_parse()?;

        Ok(AllTuples {
            macro_ident,
            start,
            end,
        })
    }
}

#[proc_macro]
pub fn product_all_tuples(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as AllTuples);
    let len = input.end - input.start;
    let mut ident_a_tuples = Vec::with_capacity(len);
    let mut ident_b_tuples = Vec::with_capacity(len);
    for i in 0..input.end{
        let ident = format_ident!("A{}", i);
        ident_a_tuples.push(ident);
        let ident = format_ident!("B{}", i);
        ident_b_tuples.push(ident);
    }

    let ident_a_tuples_ref = &ident_a_tuples;
    let ident_b_tuples_ref = &ident_b_tuples;
    let macro_ident = &input.macro_ident;
    let invocations = (input.start..input.end).flat_map(|i| {
        (input.start..input.end).map(move |j| {
            let ident_a_tuples = &ident_a_tuples_ref[..i];
            let ident_b_tuples = &ident_b_tuples_ref[..j];
            quote! {
                #macro_ident!((#(#ident_a_tuples),*); (#(#ident_b_tuples),*));
            }
        })
    });
    TokenStream::from(quote! {
        #(
            #invocations
        )*
    })
}
