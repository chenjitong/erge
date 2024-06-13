extern crate proc_macro;

use debug::debug_impl;
use default::default_new;
use display::display_impl;
use get::get_field;
use get_mut::get_mut_field;
use new::new_all;
use proc_macro::TokenStream;
use set::set_field;
use to_string::to_string_impl;
use with::with_field;

mod debug;
mod default;
mod display;
mod get;
mod get_mut;
mod new;
mod set;
mod to_string;
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

#[proc_macro_derive(New)]
pub fn new(input: TokenStream) -> TokenStream {
    new_all(input)
}

/// [`Default`] 仅支持所有成员类型都实现或者支持实现了 default 的情况.
/// 若有成员类型不支持 default，则整个结构体对于 [`Default`] 不可用，请手动提供 default 的实现
///
/// # 1.支持
/// ```rust
/// #[derive(Default)] // 整个结构体可用 Default
/// struct Stru<`a>{
///     v1 : String // 支持 default
///     v2 : bool // 支持 default
///     v3 : &'a str // 支持 default
/// }
/// ```
///
/// # 2.不支持
/// ```rust
/// struct Stru<`a>{ // 整个结构体不可用 Default
///     v1 : &'a String, // 不支持 default
///     ...
/// }
/// ```
///
#[proc_macro_derive(Default)]
pub fn default(input: TokenStream) -> TokenStream {
    default_new(input)
}

/// 为结构体提供默认的 [`Debug`] 实现
///
/// # 1.例子
/// ```rust
/// use strut_macro::Debug;
///
/// #[derive(Debug)]
/// struct Stru {
///     v1 : String,
///     v2 : bool,
/// }
///
/// let temp = Stru {
///     v1 : "say hellow".to_owned(),
///     v2 : true,
/// }
///
/// println!( "{:?}", temp );
/// ```
///
/// # 2.提供的宏展开实现代码
/// ```rust
/// impl ::std::fmt::Debug for Stru {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
///         -> std::fmt::Result {
///         let Stru { v1, v2 }: &Stru = self;
///         let name = ::core::any::type_name::<Stru>();
///         f.debug_struct(name)
///             .field("v1", &v1)
///             .field("v2", &v2)
///             .finish()
///     }
/// }
/// ```
#[proc_macro_derive(Debug)]
pub fn debug(input: TokenStream) -> TokenStream {
    debug_impl(input)
}

/// 为结构体提供默认的 [`Display`] 实现
/// # 1.__依赖于 [`Debug`] 的实现__
///     若类型未实现 [`Debug`] ，则需要手动提供 [`Display`] 的具体实现
/// # 2.提供的宏展开实现代码
/// ```rust
/// impl ::std::fmt::Display for SomeType {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
///         -> std::fmt::Result {
///         // 这里使用 Debug，默认依赖于 Debug 提供对 Display 的实现
///         (self as &dyn ::std::fmt::Debug).fmt(f)
///     }
/// }
/// ```
#[proc_macro_derive(Display)]
pub fn display(input: TokenStream) -> TokenStream {
    display_impl(input)
}

#[proc_macro_derive(ToString)]
pub fn to_string(input: TokenStream) -> TokenStream {
    TokenStream::from_iter(vec![
        debug_impl(input.clone()),
        display_impl(input.clone()),
        to_string_impl(input.clone()),
    ])
}
