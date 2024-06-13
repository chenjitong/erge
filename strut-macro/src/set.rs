use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

use crate::{
    util::field::{chk_named_st, chk_st, field_has_attr, get_field_trim, get_fields},
    NO_CHAIN, SET,
};

pub(crate) fn set_field(input: TokenStream) -> TokenStream {
    // 解析结构体的抽象语法树
    let drive_ast = parse_macro_input!(input as DeriveInput);

    // 检查是否结构体
    let ast_dt = chk_st(&drive_ast);

    // 检查是否普通结构体
    let fields = chk_named_st(ast_dt); // 成员列表

    // 取所有有效成员变量
    let filter_fields = get_fields(fields, SET);

    let st_name = &drive_ast.ident; // 结构体名

    // 结构体泛型信息：实现的 traits 的泛型参数，类型参数，泛型 where 子句限定
    let (impl_g, ty_g, where_c) = &drive_ast.generics.split_for_impl();

    // 为成员生成 set 函数
    let set_fns = filter_fields.iter().map(|f| {
        let f_name = f.ident.to_owned().unwrap(); // 成员名字
        let f_ty = f.ty.to_owned(); // 成员类型
        let f_fns_name = format_ident!("set_{}", f_name); // 成员 set 函数名

        let trim = get_field_trim(f);

        // 成员 set 函数 ast
        match field_has_attr(f, NO_CHAIN) {
            true => {
                //no chain 不支持链式调用
                quote! {
                    pub fn #f_fns_name (& mut self, #f_name : #f_ty){
                        self.#f_name = #trim
                    }
                }
            }
            false => {
                // 默认链式调用
                quote! {
                    pub fn #f_fns_name (& mut self, #f_name : #f_ty) -> &mut Self {
                        self.#f_name = #trim;
                        self
                    }
                }
            }
        }
    });

    // 为结构体生成所有成员的 set 函数实现
    TokenStream::from(quote! {
        impl #impl_g #st_name #ty_g #where_c {
            #(#set_fns)*
        }
    })
}
