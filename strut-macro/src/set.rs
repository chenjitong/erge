use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

use crate::{util::field::{chk_named_st, chk_st, get_fields}, SET};

pub (crate) fn set_field (input: TokenStream) -> TokenStream {
    // 解析结构体的抽象语法树
    let drive_ast = parse_macro_input!(input as DeriveInput);

    // 检查是否结构体
    let ast_dt = chk_st (&drive_ast);

    // 检查是否普通结构体
    let fields = chk_named_st (ast_dt); // 成员列表
    let filter_fields = get_fields (fields, SET);

    let st_name = &drive_ast.ident; // 结构体名

    // 结构体泛型信息：实现的 traits 的泛型参数，类型参数，泛型 where 子句限定
    let (impl_g, ty_g, where_c) = &drive_ast.generics.split_for_impl ();

    // 为成员生成 set 函数
    let set_fns = filter_fields.iter ().map (|f| {
        let f_name = f.ident.to_owned ().unwrap (); // 成员名字
        let f_ty = f.ty.to_owned (); // 成员类型
        let f_fns_name = format_ident!("set_{}", f_name); // 成员 set 函数名

        //match f.attrs.iter ().find (|c| c.path ().is_ident ("Chain")) {
        //     Some (_) => {
        //         return quote! {
        //             pub fn #f_fns_name (& mut self, #f_name : #f_ty) -> & mut Self {
        //                 self.#f_name = #f_name;
        //                 self
        //             }
        //         }
        //     }
        //     _ => {
        //         return quote! {
        //             pub fn #f_fns_name (& mut self, #f_name : #f_ty){
        //                 self.#f_name = #f_name
        //             }
        //         }
        //     }
        // }

        //for example
        //f.attrs.iter ().for_each (|a| {
        //     if a.path ().is_ident ("Mut") {
        //         match &a.meta {
        //             Meta::NameValue (ap) => {
        //                 match &ap.value {
        //                     // Expr::Path (v) => {
        //                     //     println!("cjt {:}", v.to_token_stream ())
        //                     // }
        //                     Expr::Lit (v) => match &v.lit {
        //                         Lit::Str (s) => println!("cjt {:}", s.value ()),
        //                         _ => todo!(),
        //                     },
        //                     _ => todo!(),
        //                 }
        //             }
        //             Meta::List (l) => {
        //                 //let tokens = & l.tokens;
        //                 //let i = tokens.into_iter ();
        //                 //i.for_each (|ii|println!("cjtkk {:?}", ii.to_token_stream ()));
        //                 //let _ = l.parse_args ();
        //                 //println!("cjt1 {:?}", i.);
        //                 match l.delimiter {
        //                     syn::MacroDelimiter::Paren (_) => println!("Paren"),
        //                     syn::MacroDelimiter::Brace (_) => println!("Brace"),
        //                     syn::MacroDelimiter::Bracket (_) => println!("ok"),
        //                 };
        //                 let _ = l.parse_nested_meta (|lg| {
        //                     println!("cjt kfjdsk {:?}", lg.path.get_ident ().unwrap ().to_token_stream ()); //
        //                     //let kkkk = lg.path.get_ident ().unwrap ();
        //                     Ok (())
        //                     // Ok (())
        //                 });
        //                 //let kkk = l.parse_args ().unwrap ();
        //                 //println!("cjt kkk {:?}", kkk.to_string ());
        //             }
        //             _ => println!("nothing"),
        //         }
        //         //println!("cjt {:?}", a.meta.require_name_value ().unwrap ().value);
        //     }
        // });

        // 成员 set 函数 ast
        quote! {
            pub fn #f_fns_name (& mut self, #f_name : #f_ty){
                self.#f_name = #f_name
            }
        }
    });

    // 为结构体生成所有成员的 set 函数实现
    TokenStream::from (quote! {
        impl #impl_g #st_name #ty_g #where_c {
            #(#set_fns)*
        }
    })
}
