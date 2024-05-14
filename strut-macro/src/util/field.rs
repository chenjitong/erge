use syn::{
    Data, DataStruct, DeriveInput, Fields, FieldsNamed,
};

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
        Fields::Named (f) => f,
        _ => panic!("Only nominal struct are supported, tuple and unit type are forbidden !!!"),
    }
}
