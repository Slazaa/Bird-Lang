use crate::parser::exprs::r#impl::Impl;

use super::block;

pub fn infer<'a>(input: &Impl<'a>) -> Result<Impl<'a>, String> {
	Ok(Impl {
		body: block::infer(&input.body)?,
		..input.clone()
	})
}