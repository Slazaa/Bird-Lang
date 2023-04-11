use crate::{parser::exprs::struct_decl::{StructDecl, Field}, transpiler::c::{r#type, ident}};

pub fn transpile_field(input: &Field) -> String {
	let ident = ident::transpile(input.ident.as_ref().unwrap());
	let r#type = r#type::transpile(&input.r#type);	
	
	format!("{type} {ident}")
}

pub fn transpile_fields(input: &[Field]) -> String {
	let mut res = "{".to_owned();

	for field in input {
		res += &format!("{};", transpile_field(field));
	}

	res += "}";

	res
}

pub fn transpile(input: &StructDecl) -> String {
	let fields = transpile_fields(input.fields.as_ref().unwrap());
	format!("struct {fields}")
}