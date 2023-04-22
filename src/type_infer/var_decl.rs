use crate::parser::exprs::var_decl::VarDecl;

pub fn infer<'a>(input: &VarDecl<'a>) -> Result<VarDecl<'a>, String> {
	let value = match &input.value {
		Some(x) => x,
		None => return Err("Expected Some value, found None".to_owned())
	};

	Ok(VarDecl {
		r#type: Some(super::infer_from_value(value)),
		value: Some(super::infer(value)?),
		..input.clone()
	})
}