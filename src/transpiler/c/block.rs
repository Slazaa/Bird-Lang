use crate::parser::exprs::{block::Block, Expr};

pub fn transpile(input: &Block) -> String {
	let mut exprs = String::new();

	for expr in &input.exprs {
		exprs += &super::transpile(expr);

		match expr {
			Expr::Block(_) |
			Expr::If(_) |
			Expr::While(_) => { },
			_ => exprs += ";"
		}
	}    

	format!("{{{}}}", exprs)
}