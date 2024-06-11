extern crate proc_macro;

use get::get_field;
use get_mut::get_mut_field;
use proc_macro::TokenStream;
use set::set_field;
use with::with_field;

mod get;
mod get_mut;
mod set;
mod util;
mod with;

const SET: &str = "Set";
const GET: &str = "Get";
const MUT: &str = "Mut";
const WITH: &str = "With";
const SKIP: &str = "Skip";
const TRIM: &str = "Trim";
const NO_CHAIN: &str = "NoChain";
const ATTRS: [&str; 3] = [SKIP, TRIM, NO_CHAIN];
const SKIP_ENABLE: [&str; 4] = [SET, GET, MUT, WITH];

const TRIM_STRING: &str = "String";
const TRIM_STR: &str = "str";

#[proc_macro_derive(Set, attributes(Skip, Trim, NoChain))]
pub fn setter(input: TokenStream) -> TokenStream {
    set_field(input)
}

#[proc_macro_derive(Get, attributes(Skip))]
pub fn getter(input: TokenStream) -> TokenStream {
    get_field(input)
}

#[proc_macro_derive(Mut, attributes(Skip))]
pub fn getter_mut(input: TokenStream) -> TokenStream {
    get_mut_field(input)
}

#[proc_macro_derive(With, attributes(Skip, Trim))]
pub fn wither(input: TokenStream) -> TokenStream {
    with_field(input)
}
