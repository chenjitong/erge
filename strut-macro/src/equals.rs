use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::util::field::{chk_named_st, chk_st};

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

    let refs_a_pe = refs_a.clone();
    let refs_b_pe = refs_b.clone();

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

    // 实现的 equals 的函数体
    let equals = quote! {
        impl #impl_g for #st_name #ty_g #where_c {
            fn equals(&self, other : &#st_name #ty_g) -> {
                self.eq(&other)
            }
        }
    };

    // 实现的 eq 的函数体
    let eq = quote! {
        impl #impl_g std::core:;cmp::PartialEq for #st_name #ty_g #where_c {
            fn eq(&self, other: &#st_name #ty_g) -> bool{
                match *other{
                    #st_name {
                        #(#refs_b)*
                    } => match *self{
                        #st_name {
                            #(#refs_a)*
                        }=>{
                            #(#eq_condition)*
                            true
                        }
                    },
                }
            }
        }
    };

    // 实现的 ne 的函数体
    let ne = quote! {
        impl #impl_g std::core:;cmp::PartialEq for #st_name #ty_g #where_c {
            fn ne(&self, other : &#st_name #ty_g) -> bool{
                match *other {
                    #st_name {
                        #(#refs_b_pe)*
                    }=>match *self{
                        #st_name{
                            #(#refs_a_pe)*
                        }=>{
                            #(#ne_condition)*
                            false
                        }
                    },
                }
            }
        }
    };

    TokenStream::from(quote! {
        #equals
        #eq
        #ne
    })
}
