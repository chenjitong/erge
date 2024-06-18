use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::{
    util::field::{chk_named_st, chk_st, get_fields, get_skip_fields},
    DISPLAY,
};

/// 为结构体提供默认的 [`Display`] 实现
///
/// # Panics
///
/// Panics if 解析结构体元代码或宏展开时出错
///
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
pub(crate) fn display_impl(input: TokenStream) -> TokenStream {
    // 解析结构体的抽象语法树
    let drive_ast = parse_macro_input!(input as DeriveInput);

    // 检查是否结构体
    let ast_dt = chk_st(&drive_ast);

    // 结构体名
    let st_name = &drive_ast.ident;

    // 检查是否普通结构体
    let fields = chk_named_st(ast_dt); // 成员列表

    // 取所有有效成员变量
    let filter_fields = get_fields(fields, DISPLAY);

    // 取所有有效成员变量
    let skip_fields = get_skip_fields(fields, DISPLAY);

    // 结构体泛型信息：实现的 traits 的泛型参数，类型参数，泛型 where 子句限定
    let (impl_g, ty_g, where_c) = &drive_ast.generics.split_for_impl();

    // 结构体有效成员模式识别
    let refs = filter_fields.iter().map(|f| {
        let fname = f.ident.clone().unwrap(); // 成员变量
        quote! {#fname,}
    });

    // 结构体略过成员模式识别
    let refs_skiped = skip_fields.iter().map(|f| {
        let fname = f.ident.clone().unwrap();
        quote! {#fname : _,}
    });

    // display 的 formatter 对每个非 skip 成员变量的处理
    let fields_fmt = filter_fields.iter().map(|f| {
        let fname = f.ident.clone().unwrap();
        let fname_str = fname.to_string();
        quote! {
            .field(#fname_str, &#fname )
        }
    });

    // 实现的 display 的 fmt 函数的函数体
    let display_fmt = quote! {
        let #st_name{#(#refs)*#(#refs_skiped)*} : &#st_name = self;
        let name = ::core::any::type_name::<#st_name>();
        f.debug_struct(name)
            #(#fields_fmt)*
            .finish()
    };

    TokenStream::from(quote! {
        impl #impl_g ::std::fmt::Display for #st_name #ty_g #where_c {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                #display_fmt
            }
        }
    })
}
