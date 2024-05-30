extern crate proc_macro;

use get::get_field;
use proc_macro::TokenStream;
use set::set_field;

mod get;
mod set;
mod util;

#[proc_macro_derive(Set, attributes(Skip, Mut))]
pub fn setter(input: TokenStream) -> TokenStream {
    set_field(input)
}

#[proc_macro_derive(Get, attributes(Skip))]
pub fn getter(input: TokenStream) -> TokenStream {
    get_field(input)
}
