use crate::parser::exprs::box_decl::BoxDecl;

pub fn infer<'a>(input: &BoxDecl<'a>) -> Result<BoxDecl<'a>, String> {
	let value = match &input.value {
		Some(x) => x,
		None => return Err("Expected Some value, found None".to_owned())
	};

	Ok(BoxDecl {
		r#type: Some(super::infer_from_value(value)),
		value: Some(super::infer(value)?),
		..input.clone()
	})
}