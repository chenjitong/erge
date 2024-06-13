use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::util::field::{chk_named_st, chk_st};
pub(crate) fn default_new(input: TokenStream) -> TokenStream {
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

    // 所有结构体成员
    let all_fields = fields.named.iter().map(|f| {
        let fname = f.ident.clone().unwrap(); // 成员名
        quote! {
            #fname: ::core::default::Default::default(),
        }
    });

    // 缺省
    let impl_default = quote! {
        impl #impl_g ::core::default::Default for #st_name #ty_g #where_c {
            fn default () -> Self {
                Self {
                    #(
                        #all_fields
                    )*
                }
            }
        }

        impl #impl_g #st_name #ty_g #where_c {
            pub fn default() -> Self {
                ::core::default::Default::default()
            }
        }
    };

    TokenStream::from(quote! {#impl_default})
}
