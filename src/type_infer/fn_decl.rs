use crate::parser::exprs::fn_decl::FnDecl;

use super::block;

pub fn infer<'a>(input: &FnDecl<'a>) -> Result<FnDecl<'a>, String> {
	Ok(FnDecl {
		body: block::infer(&input.body)?,
		..input.clone()
	})
}