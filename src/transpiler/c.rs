use crate::parser::exprs::{Expr, file::File};

pub mod assign;
pub mod block;
pub mod file;
pub mod fn_call;
pub mod fn_decl;
pub mod ident;
pub mod literals;
pub mod r#if;
pub mod path;
pub mod r#return;
pub mod struct_decl;
pub mod r#type;
pub mod typedef;
pub mod var_decl;
pub mod r#while;

use literals::*;

const PRIMITIVES: &str = "typedef enum{false,true}bool;";

pub fn transpile(input: &Expr) -> String {
	match &input {
		Expr::Bool(expr) => bool::transpile(expr),
		Expr::Int(expr) => int::transpile(expr),
		Expr::String(expr) => string::transpile(expr),

		Expr::Assign(expr) => assign::transpile(expr),
		Expr::Block(expr) => block::transpile(expr),
		Expr::BoxDecl(expr) => var_decl::transpile(expr),
		Expr::FnCall(expr) => fn_call::transpile(expr),
		Expr::Ident(expr) => ident::transpile(expr),
		Expr::If(expr) => r#if::transpile(expr),
		Expr::Path(expr) => path::transpile(expr),
		Expr::StructDecl(expr) => struct_decl::transpile(expr),
		Expr::Return(expr) => r#return::transpile(expr),
		Expr::While(expr) => r#while::transpile(expr),

		_ => todo!("{:?}", input)
	}
}

pub fn transpile_file(input: &File) -> String {
	PRIMITIVES.to_owned() + &file::transpile(input)
}
