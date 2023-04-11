use crate::{parser::exprs::box_decl::BoxDecl, transpiler::c::{r#type, ident}};

pub fn transpile(input: &BoxDecl) -> String {
    let r#type = r#type::transpile(input.r#type.as_ref().unwrap());
    let ident = ident::transpile(&input.ident);

    if let Some(value) = &input.value {
        let value = super::transpile(value);
        format!("{type} {ident}={value};")
    } else {
        format!("{type} {ident};")
    }
}