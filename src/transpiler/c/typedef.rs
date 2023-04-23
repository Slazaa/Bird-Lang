use crate::parser::exprs::box_decl::BoxDecl;

use super::{r#type, ident};

pub fn transpile(input: &BoxDecl) -> Result<String, ()> {
	let r#type = match &input.r#type {
		Some(x) => x,
		None => return Err(())
	};

	if r#type::transpile(r#type) != "type" {
		return Err(());
	}

	let value = match &input.value {
		Some(x) => x,
		None => return Err(())
	};

	let from = super::transpile(value);
	let to = ident::transpile(&input.ident);

	Ok(format!("typedef {from} {to}"))
}