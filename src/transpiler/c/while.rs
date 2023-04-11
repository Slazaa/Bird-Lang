use crate::{parser::exprs::r#while::While, transpiler::c::block};

pub fn transpile(input: &While) -> String {
	let cond = super::transpile(&input.cond);
	let body = block::transpile(&input.body);

	format!("while({cond}){body}")
}