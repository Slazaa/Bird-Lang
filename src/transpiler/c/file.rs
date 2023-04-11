use crate::parser::exprs::{Expr, file::File};

use super::{fn_decl, typedef};

pub fn transpile(input: &File) -> String {
	let mut fn_sigs = String::new();
	let mut res = String::new();

	for expr in &input.exprs {
		match expr {
			Expr::BoxDecl(box_decl) => if let Ok(transpiled) = fn_decl::transpile(box_decl) {
				if box_decl.ident.value != "main" {
					fn_sigs += &fn_decl::transpile_sig(box_decl).unwrap();
				}

				res += &transpiled;
			} else if let Ok(transpiled) = typedef::transpile(box_decl) {
				res += &transpiled;
			} else {
				res += &super::transpile(expr);
			},
			_ => res += &super::transpile(expr)
		}

		res += ";";
	}

	fn_sigs + &res
}
