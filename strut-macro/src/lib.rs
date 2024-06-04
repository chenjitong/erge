extern crate proc_macro;

use get::get_field;
use proc_macro::TokenStream;
use set::set_field;

mod get;
mod set;
mod util;

const SET : &str = "Set";
const GET : &str = "Get";
const SKIP : &str = "Skip";
const TRIM : &str = "Trim";
const ATTRS: [&str; 2] = [SKIP, TRIM];
const TRIM_TYPES: [&str; 1] = ["String"];
const SKIP_ENABLE: [&str; 2] = [SET, GET];

#[proc_macro_derive(Set, attributes(Skip, Trim))]
pub fn setter(input: TokenStream) -> TokenStream {
    set_field(input)
}

#[proc_macro_derive(Get, attributes(Skip))]
pub fn getter(input: TokenStream) -> TokenStream {
    get_field(input)
}
