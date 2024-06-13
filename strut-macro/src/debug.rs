use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::util::field::{chk_named_st, chk_st};

/// 为结构体提供默认的 [`Debug`] 实现
///
/// # Panics
///
/// Panics if 解析结构体元代码或宏展开时出错
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
pub(crate) fn debug_impl(input: TokenStream) -> TokenStream {
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

    // 结构体成员模式识别
    let refs = fields.named.iter().map(|f| {
        let fname = f.ident.clone().unwrap(); // 成员变量
        quote! {#fname,}
    });

    // debug 的 formatter 对每个成员变量的处理
    let fields_fmt = fields.named.iter().map(|f| {
        let fname = f.ident.clone().unwrap();
        let fname_str = fname.to_string();
        quote! {
            .field(#fname_str, &#fname )
        }
    });

    // 实现的 debug 的 fmt 函数的函数体
    let debug_fmt = quote! {
        let #st_name{#(#refs)*} : &#st_name = self;
        let name = ::core::any::type_name::<#st_name>();
        f.debug_struct(name)
            #(#fields_fmt)*
            .finish()
    };

    TokenStream::from(quote! {
        impl #impl_g ::std::fmt::Debug for #st_name #ty_g #where_c {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                #debug_fmt
            }
        }
    })
}
