use crate::parser::exprs::{Expr, file::File};

pub mod file;
pub mod fn_decl;
pub mod ident;
pub mod path;
pub mod r#type;

pub fn transpile(input: &Expr) -> String {
	match &input {
		Expr::Ident(expr) => ident::transpile(expr),
		_ => todo!()
	}
}

pub fn transpile_file(input: &File) -> String {
	file::transpile(input)
}
