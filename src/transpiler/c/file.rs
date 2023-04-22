use crate::parser::exprs::{file::File, Expr};

use super::fn_decl;

pub fn transpile(input: &File) -> String {
	let mut fn_sigs = String::new();
	let mut res = String::new();

	for expr in &input.exprs {
		if let Expr::FnDecl(expr) = expr {
			fn_sigs += &fn_decl::transpile_sig(expr);
		}

		res += &super::transpile(expr);
	}

	fn_sigs + &res
}
