use crate::parser::exprs::{Expr, file::File};

pub mod block;
pub mod file;
pub mod fn_decl;
pub mod ident;
pub mod literals;
pub mod r#if;
pub mod path;
pub mod r#type;
pub mod var_decl;
pub mod r#while;

use literals::*;

pub fn transpile(input: &Expr) -> String {
	match &input {
		Expr::Bool(expr) => bool::transpile(expr),
		Expr::Int(expr) => int::transpile(expr),

		Expr::Block(expr) => block::transpile(expr),
		Expr::BoxDecl(expr) => var_decl::transpile(expr),
		Expr::Ident(expr) => ident::transpile(expr),
		Expr::If(expr) => r#if::transpile(expr),
		Expr::While(expr) => r#while::transpile(expr),

		_ => todo!("{:?}", input)
	}
}

pub fn transpile_file(input: &File) -> String {
	file::transpile(input)
}
