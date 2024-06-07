extern crate proc_macro;

use get::get_field;
use get_mut::get_mut_field;
use proc_macro::TokenStream;
use set::set_field;

mod get;
mod set;
mod get_mut;
mod util;

const SET : &str = "Set";
const GET : &str = "Get";
const MUT : &str = "Mut";
const SKIP : &str = "Skip";
const TRIM : &str = "Trim";
const NO_CHAIN :&str = "NoChain";
const ATTRS: [&str; 3] = [SKIP, TRIM, NO_CHAIN];
// const TRIM_TYPES: [&str; 1] = ["String"];
const SKIP_ENABLE: [&str; 3] = [SET, GET, MUT];

#[proc_macro_derive(Set, attributes(Skip, Trim, NoChain))]
pub fn setter(input: TokenStream) -> TokenStream {
    set_field(input)
}

#[proc_macro_derive(Get, attributes(Skip, Trim))]
pub fn getter(input: TokenStream) -> TokenStream {
    get_field(input)
}

#[proc_macro_derive(Mut, attributes(Skip))]
pub fn getter_mut(input: TokenStream) -> TokenStream {
    get_mut_field(input)
}
