use crate::parser::exprs::block::Block;

pub fn infer<'a>(input: &Block<'a>) -> Result<Block<'a>, String> {
	let mut exprs = Vec::new();

	for expr in &input.exprs {
		exprs.push(super::infer(expr)?);
	}

	Ok(Block { exprs })
}