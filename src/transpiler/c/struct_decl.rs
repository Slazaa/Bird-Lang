use crate::{parser::exprs::struct_decl::{StructDecl, Field}, transpiler::c::{r#type, ident}};

pub fn transpile_field(input: &Field, next_anon_field: &mut i32) -> String {
	let ident = if let Some(ident) = &input.ident {
		ident::transpile(ident)
	} else {
		let res = format!("{}", next_anon_field) + "_";
		*next_anon_field += 1;
		res 
	};

	let r#type = r#type::transpile(&input.r#type);	
	
	format!("{type} {ident}")
}

pub fn transpile_fields(input: &[Field]) -> String {
	let mut res = "{".to_owned();
	let mut next_anon_field = 0;

	for field in input {
		res += &format!("{};", transpile_field(field, &mut next_anon_field));
	}

	res += "}";

	res
}

pub fn transpile(input: &StructDecl) -> String {
	let fields = if let Some(fields) = input.fields.as_ref() {
		transpile_fields(fields)
	} else {
		"{}".to_owned()
	};

	format!("struct {fields}")
}