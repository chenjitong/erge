use core::panic;

use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed};

use crate::{ATTRS, SKIP, SKIP_ENABLE};

/// 判断是否为结构体，是则返回 ast 的 DataStruct
///
/// # Panics
///
/// Panics if 非结构体
pub (crate) fn chk_st (ast: &DeriveInput) -> &DataStruct {
    match &ast.data {
        Data::Struct (s) => s,
        _ => panic!("Only Struct type are supported !!!"),
    }
}

/// 判断是否是普通结构体，是则返回结构体的命名成员列表
///
/// # Panics
///
/// Panics if 非普通结构体
pub (crate) fn chk_named_st (ast_dt: &DataStruct) -> &FieldsNamed {
    match &ast_dt.fields {
        Fields::Named (f) => {
            f.named.iter ().for_each (|field| {
                chk_field_attr (field);
            });
            f
        }
        _ => panic!("Only nominal struct are supported, tuple and unit type are forbidden !!!"),
    }
}

pub (crate) fn chk_field_attr (field: &Field) -> bool {
    if field.attrs.len () == 0 {
        // 可以不设属性
        return true;
    };

    // 同一属性只能定义一次
    ATTRS.iter ().for_each (|c| {
        if field.attrs.iter ().filter (|a| a.path ().is_ident (c)).count () > 1 {
            panic!("The same attribute only can be used once");
        }
    });

    // 属性必须在 ATTRS 范围内
    field.attrs.iter ().for_each (|attr| {
        if !ATTRS.iter ().any (|c| attr.path ().is_ident (c)) {
            panic!("Invalid attribute，attribute must be in {:?}", ATTRS);
        }
    });
    true
}

pub (crate) fn chk_field_skip (field: &Field, ident: &str) -> bool {
    // 检查 skip attribute 支持的派生宏范围
    if !SKIP_ENABLE.iter ().any (|i| ident.eq (*i)) {
        panic!("Skip attribute only enabled for {:?}", SKIP_ENABLE);
    }

    if field.attrs.len () == 0 {
        //no any attributes means no skip
        return false;
    };

    //no skip
    if field
        .attrs
        .iter ()
        .filter (|attr| attr.path ().is_ident (SKIP))
        .count ()
        < 1
    {
        return false;
    }

    // 返回是否 skip
    field.attrs.iter ().any (|attr| {
        match &attr.meta {
            syn::Meta::List (list) => match &list.delimiter {
                syn::MacroDelimiter::Bracket (_) => {
                    list.parse_nested_meta (|id| {
                        if id.path.is_ident (ident){
                            true
                        }
                        return Ok (());
                    });
                },
                _ => panic!("Unexpected macro delimiter"),
            },
            _ => panic!("Skip attribute"),
        }
        //attr.path ().is_ident (ident)
    })
}

pub (crate) fn get_fields<'a>(fields: &'a FieldsNamed, ident: &'a str) -> Vec<&'a Field> {
    fields
        .named
        .iter ()
        .filter (|field| {
            chk_field_skip (field, ident)
            //field.attrs.len () == 0
            //     || field
            //         .attrs
            //         .iter ()
            //         .filter (|at| at.path ().is_ident (SKIP))
            //         .count ()
            //         == 0
        })
        .map (|f| f)
        .collect ()
    //let vec: &Vec<&Field> = &res;
    //vec
    //for k in new_fields {
    //     println!("chenjitong {:?}", k.ident.to_owned ());
    // }
    //let fields_new : Vec<&Field> = vec![];
    //filter_fields.for_each (move|f| fields_new.push (f));
    //fields_new
    //fields
    //new_fields.for_each (move|f| println!("kkk {:?}", f.ident.to_owned ()));
}
