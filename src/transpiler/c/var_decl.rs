use crate::{parser::exprs::box_decl::BoxDecl, transpiler::c::{r#type, ident}};

pub fn transpile(input: &BoxDecl) -> String {
	let r#type = r#type::transpile(input.r#type.as_ref().unwrap());
	let ident = ident::transpile(&input.ident);

	let r#const = if !input.r#mut {
		"const "
	} else {
		""
	};

	if let Some(value) = &input.value {
		let value = super::transpile(value);
		format!("{type} {const}{ident}={value}")
	} else {
		format!("{type} {ident}")
	}
}