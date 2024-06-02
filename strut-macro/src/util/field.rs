use syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed};

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
        Fields::Named(f) => f,
        _ => panic!("Only nominal struct are supported, tuple and unit type are forbidden !!!"),
    }
}

pub(crate) fn filter_skip_named_fields(fields: &FieldsNamed) -> Vec<&Field> {
    fields
        .named
        .iter()
        .filter(|field| {
            field.attrs.len() == 0
                || field
                    .attrs
                    .iter()
                    .filter(|at| at.path().is_ident("Skip"))
                    .count()
                    == 0
        })
        .map(|f| f)
        .collect()
    // let vec: &Vec<&Field> = &res;
    // vec
    //for k in new_fields {
    //     println!("chenjitong {:?}", k.ident.to_owned ());
    // }
    // let fields_new : Vec<&Field> = vec![];
    // filter_fields.for_each(move|f| fields_new.push(f));
    // fields_new
    // fields
    //new_fields.for_each (move|f| println!("kkk {:?}", f.ident.to_owned ()));
}
