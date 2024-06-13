use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::util::field::{chk_named_st, chk_st};

/// 为结构体提供默认的 [`to_string`] 实现
/// to_string 会自动提供默认的 [`Debug`] 与 [`Display`] 的实现
/// 若需要提供特殊的 [`Debug`]，[`Display`] 与 [`ToString`], 请手动自行实现
///
/// # Panics
///
/// Panics if 解析结构体元代码或宏展开时出错
///
/// # 例子
/// ```rust
/// impl ::std::fmt::Display for SomeType {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>)
///         -> std::fmt::Result {
///         // 这里使用 Debug，默认依赖于 Debug 提供对 Display 的实现
///         (self as &dyn ::std::fmt::Debug).fmt(f)
///     }
/// }
/// ```
pub(crate) fn to_string_impl(input: TokenStream) -> TokenStream {
    // 解析结构体的抽象语法树
    let drive_ast = parse_macro_input!(input as DeriveInput);

    // 检查是否结构体
    let ast_dt = chk_st(&drive_ast);

    // 结构体名
    let st_name = &drive_ast.ident;

    // 检查是否普通结构体
    let _fields = chk_named_st(ast_dt); // 成员列表

    // 结构体泛型信息：实现的 traits 的泛型参数，类型参数，泛型 where 子句限定
    let (impl_g, ty_g, where_c) = &drive_ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_g #st_name #ty_g #where_c {
            fn to_string(&self) -> ::std::string::String {
                ::std::format!("{}", self)
            }
        }
    })
}
