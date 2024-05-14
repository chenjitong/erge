mod tests;
use proc_macro::TokenStream;
use set::set_filename;

mod set;
mod util;
extern crate proc_macro;

#[proc_macro_derive(Set)]
pub fn derive_set(input:TokenStream)->TokenStream{
    set_filename(input)
}
