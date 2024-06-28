use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::{
    util::field::{chk_named_st, chk_st, get_fields, get_skip_fields},
    DISPLAY,
};

/// 为结构体提供默认的 [`Equals`] 实现
///
/// # Panics
///
/// Panics if 解析结构体元代码或宏展开时出错
pub(crate) fn equals_impl(input: TokenStream) -> TokenStream {
    // 解析结构体的抽象语法树
    let drive_ast = parse_macro_input!(input as DeriveInput);

    // 检查是否结构体
    let ast_dt = chk_st(&drive_ast);

    // 结构体名
    let st_name = &drive_ast.ident;

    // 检查是否普通结构体
    let fields = chk_named_st(ast_dt); // 成员列表

    // 结构体泛型信息：实现的 traits 的泛型参数，类型参数，泛型 where 子句限定
    let (impl_g, ty_g, where_c) = &drive_ast.generics.split_for_impl();

    // 结构体成员模式识别 for A
    let refs_a = fields.named.iter().map(|f| {
        let fname = f.ident.clone().unwrap(); // 成员变量
        quote! {#fname,}
    });

    // 结构体成员模式识别 for B
    let refs_b = fields.named.iter().map(|f| {
        let fname = f.ident.clone().unwrap();
        quote! {#fname : _,}
    });

    let eq_condition = fields.named.iter().map(|f| {
        let fname = f.ident.clone().unwrap();
        quote! {
            (*self.#fname) == (*other.#fname) &&
        }
    });
    let ne_condition = fields.named.iter().map(|f| {
        let fname = f.ident.clone().unwrap();
        quote! {
            (*self.#fname) != (*other.#fname) ||
        }
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
