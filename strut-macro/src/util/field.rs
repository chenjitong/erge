use core::panic;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Type};

use crate::{ATTRS, SKIP, SKIP_ENABLE, TRIM, TRIM_STR, TRIM_STRING};

/// 判断是否为结构体，是则返回 ast 的 DataStruct
///
/// # Panics
///
/// Panics if 非结构体
pub(crate) fn chk_st(ast: &DeriveInput) -> &DataStruct {
    match &ast.data {
        Data::Struct(s) => s,
        _ => panic!("Only Struct type are supported !!!"),
    }
}

/// 判断是否是普通结构体，是则返回结构体的命名成员列表
///
/// # Panics
///
/// Panics if 非普通结构体
pub(crate) fn chk_named_st(ast_dt: &DataStruct) -> &FieldsNamed {
    match &ast_dt.fields {
        Fields::Named(f) => {
            f.named.iter().for_each(|field| {
                chk_field_attr(field); // 检查各成员派生宏辅助帮助属性宏
            });
            f
        }
        _ => panic!("Only nominal struct are supported, tuple and unit type are forbidden !!!"),
    }
}

/// 检查各成员变量派生宏附属帮助属性宏
///
/// # Panics
///
/// Panics if the same attribute was settled more than once
pub(crate) fn chk_field_attr(field: &Field) -> bool {
    if field.attrs.is_empty() {
        // 可以不设属性
        return true;
    };

    // 本库中，同一属性只能定义一次
    ATTRS.iter().for_each(|c| {
        if field.attrs.iter().filter(|a| a.path().is_ident(c)).count() > 1 {
            panic!("The same attribute {:?} only can be used once", c);
        }
    });

    // 属性必须在 ATTRS 范围内，去除，否则可能和其他依赖属性宏不兼容
    ////field.attrs.iter ().for_each (|attr| {
    ////     if !ATTRS.iter ().any (|c| attr.path ().is_ident (c)) {
    ////         panic!("Invalid attribute，attribute must be in {:?}", ATTRS);
    ////     }
    //// });
    true
}

/// 检查派生宏是否在成员变量上 skip，如果 true 表明 skip，false 表示不忽略
///
/// # Panics
///
/// Panics if 辅助帮助属性格式不是 #[Skip] 或者 #[Skip [Set, Get]] 格式
pub(crate) fn is_skip_ident(field: &Field, ident: &str) -> bool {
    // 检查 skip attribute 支持的派生宏范围
    if !SKIP_ENABLE.iter().any(|i| ident.eq(*i)) {
        panic!("Skip attribute only enabled for {:?}", SKIP_ENABLE);
    }

    if field.attrs.is_empty() {
        //no any attributes means no skip
        return false;
    };

    let skip_op = field.attrs.iter().find(|attr| attr.path().is_ident(SKIP));
    if skip_op.is_none() {
        return false;
    } //no skip
    let skip = skip_op.unwrap();

    //skip 无标注派生宏范围，means all skipped
    let list_op = skip.meta.require_list();
    if list_op.is_err() {
        return true; //skip all
    }
    let list = list_op.unwrap();

    //check 格式 Skip [Set, ... , Get]， 其他格式 Skip (Set, ..., Get) 等均非法
    match &list.delimiter {
        syn::MacroDelimiter::Bracket(_) => list,
        _ => panic!("Unexpected macro delimiter"),
    };

    //skip attributes 是否标注当前派生宏 skip
    let mut rs = false;
    let _ = list.parse_nested_meta(|logic| {
        if logic.path.is_ident(ident) {
            rs = true; //skip
        }
        Ok(())
    });
    rs
}

/// 获取成员 trim 元代码语句
///
/// # Panics
///
/// Panics if 不支持 trim 的成员类型
pub(crate) fn get_field_trim(field: &Field) -> TokenStream {
    let f_name = field.ident.to_owned().unwrap();

    if !field_has_attr(field, TRIM) {
        return quote! {#f_name};
    }

    match &field.ty {
        Type::Path(x) => {
            if x.qself.is_none() {
                if x.path.is_ident(TRIM_STRING) {
                    quote! {
                        #f_name.trim().to_string()
                    }
                } else {
                    panic!("Trim Attribute can only support [String] [&str] type!");
                }
            } else {
                panic!("Trim Attribute can't support qself style!");
            }
        }
        Type::Reference(x) => match *x.elem.clone() {
            Type::Path(e) => {
                if e.path.is_ident(TRIM_STR) {
                    quote! {
                        #f_name.trim()
                    }
                } else {
                    panic!("Trim Attribute can only support [String] [&str] type!");
                }
            }
            _ => panic!("Trim Attribute can only support [String] [&str] type!"),
        },
        _ => panic!("Trim Attribute can only support [String] [&str] type!"),
    }
}

/// 检查一个成员是否被标注给定的属性宏
///
/// # Panics
///
/// Panics if 给定的属性宏不在允许范围内
pub(crate) fn field_has_attr(field: &Field, attr: &str) -> bool {
    // 检查 attribute 支持宏范围
    if !ATTRS.iter().any(|i| attr.eq(*i)) {
        panic!("Attribute only enabled for {:?}", ATTRS);
    }

    if field.attrs.is_empty() {
        //no any attributes means no
        return false;
    };

    let attr_op = field.attrs.iter().find(|at| at.path().is_ident(attr));
    if attr_op.is_none() {
        return false;
    } //no
    true
}

/// 取得当前派生宏生效的成员变量列表
pub(crate) fn get_fields<'a>(fields: &'a FieldsNamed, ident: &'a str) -> Vec<&'a Field> {
    fields
        .named
        .iter()
        .filter(|field| !is_skip_ident(field, ident))
        .collect()
}

/// 取得当前派生宏略过的成员变量列表
pub(crate) fn get_skip_fields<'a>(fields: &'a FieldsNamed, ident: &'a str) -> Vec<&'a Field> {
    fields
        .named
        .iter()
        .filter(|field| is_skip_ident(field, ident))
        .collect()
}
