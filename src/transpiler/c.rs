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

const PRIMITIVES: &str = "\
typedef enum{false,true}bool;\
\
typedef i8 signed char;\
typedef i16 signed short;\
typedef i32 signed int;\
typedef i64 signed long;\
\
typedef u8 unsigned char;\
typedef u16 unsigned short;\
typedef u32 unsigned int;\
typedef u64 unsigned long;\
\
typedef char_ u32;\
typedef str const u8*;\
";

pub fn transpile(input: &Expr) -> String {
	match &input {
		Expr::Bool(expr) => bool::transpile(expr),
		Expr::Char(expr) => char::transpile(expr),
		Expr::Float(expr) => float::transpile(expr),
		Expr::Int(expr) => int::transpile(expr),

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
