use core::panic;
use std::ops::Deref;

use quote::ToTokens;
use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Type};

use crate::{ATTRS, NO_CHAIN, SKIP, SKIP_ENABLE};

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
    if field.attrs.len() == 0 {
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

    if field.attrs.len() == 0 {
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

#[allow(dead_code)]
pub(crate) fn is_field_trim(field: &Field) -> bool {
    println!("{:?}\n", field.to_token_stream());
    match &field.ty {
        Type::Array(_) => println!("is 1"),
        Type::BareFn(_) => println!("is 2"),
        Type::Group(_) => println!("is 3"),
        Type::ImplTrait(_) => println!("is 4"),
        Type::Infer(_) => println!("is 5"),
        Type::Macro(_) => println!("is 6"),
        Type::Never(_) => println!("is 7"),
        Type::Paren(_) => println!("is 8"),
        Type::Path(x) => {
            if x.qself.is_none() {
                println!(
                    "is 9 {:?} {:?} {:?}",
                    x.path.get_ident().to_token_stream(),
                    field.ident.to_token_stream(),
                    x.path.segments.to_token_stream()
                );
                x.path.segments.iter().for_each(|s| {
                    println!("in this loop: {:?}", s.ident.to_string());
                    match &s.arguments {
                        syn::PathArguments::AngleBracketed(a) => {
                            println!("analeBracketed\n");
                            a.args.iter().for_each(|args| match &args {
                                syn::GenericArgument::Lifetime(_) => println!("lifetime"),
                                syn::GenericArgument::Type(t) => match t {
                                    Type::Path(p) => {
                                        println!("{:}", p.path.get_ident().to_token_stream());
                                    }
                                    Type::Reference(r) => match r.elem.deref() {
                                        Type::Array(_) => println!("array"),
                                        Type::BareFn(_) => println!("barefn"),
                                        Type::Group(_) => println!("group"),
                                        Type::ImplTrait(_) => println!("impltrait"),
                                        Type::Infer(_) => println!("infer"),
                                        Type::Macro(_) => println!("macro"),
                                        Type::Never(_) => println!("never"),
                                        Type::Paren(_) => println!("paren"),
                                        Type::Path(k) => {
                                            println!("{:}", k.path.get_ident().to_token_stream())
                                        }
                                        Type::Ptr(_) => println!("ptr"),
                                        Type::Reference(_) => println!("reference"),
                                        Type::Slice(_) => println!("slice"),
                                        Type::TraitObject(_) => println!("traitobject"),
                                        Type::Tuple(_) => println!("tuple"),
                                        Type::Verbatim(_) => println!("verbatim"),
                                        _ => println!(""),
                                    },
                                    _ => (),
                                },
                                syn::GenericArgument::Const(_) => println!("const"),
                                syn::GenericArgument::AssocType(_) => println!("assoctype"),
                                syn::GenericArgument::AssocConst(_) => println!("assocconst"),
                                syn::GenericArgument::Constraint(_) => println!("constraint"),
                                _ => todo!(),
                            });
                        }
                        _ => (),
                    }
                });
            } else {
                println!(
                    "is 9 {:?} {:?} {:?}",
                    x.path.get_ident().to_token_stream(),
                    field.ident.to_token_stream(),
                    x.qself.clone().unwrap().position.to_token_stream()
                )
            }
        }
        Type::Ptr(_) => println!("is 10"),
        Type::Reference(x) => println!(
            "is 11 {:?} {:}",
            x.elem.to_token_stream(),
            field.ident.to_token_stream()
        ),
        Type::Slice(_) => println!("is 12"),
        Type::TraitObject(_) => println!("is 13"),
        Type::Tuple(_) => println!("is 14"),
        Type::Verbatim(_) => println!("is 15"),
        _ => println!("is _"),
    };
    println!("\n");
    //let trim = field_has_attr (field, TRIM);
    //trim
    false
}

pub(crate) fn field_has_attr(field: &Field, attr: &str) -> bool {
    // 检查 attribute 支持宏范围
    if !ATTRS.iter().any(|i| attr.eq(*i)) {
        panic!("Attribute only enabled for {:?}", ATTRS);
    }

    if field.attrs.len() == 0 {
        //no any attributes means no
        return false;
    };

    let attr_op = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident(NO_CHAIN));
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
        .map(|f| f)
        .collect()
}
