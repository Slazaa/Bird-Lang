use crate::parser::exprs::{file::File, Expr, r#type::Type, path::Path, ident::Ident};

pub mod block;
pub mod var_decl;
pub mod fn_decl;

pub fn type_from(input: &str) -> Type {
	Type {
		value: Expr::Path(Path {
			exprs: vec![Expr::Ident(Ident { value: input })]
		}),
		ptr_kind: None
	}
}

pub fn infer_from_value<'a>(input: &Expr<'a>) -> Type<'a> {
	match &input {
		Expr::Bool(_) => type_from("bool"),
		Expr::Char(_) => type_from("char_"),
		Expr::Float(_) => type_from("comp_float"),
		Expr::Int(_) => type_from("comp_int"),
		Expr::String(_) => type_from("str"),

		Expr::StructDecl(_) |
		Expr::EnumDecl(_) => type_from("type"),

		_ => todo!("{:?}", input)
	}
}

pub fn infer<'a>(input: &Expr<'a>) -> Result<Expr<'a>, String> {
	Ok(match &input {
		Expr::Block(expr) => Expr::Block(Box::new(block::infer(expr)?)),
		Expr::VarDecl(expr) if expr.r#type.is_none() && expr.value.is_some() => Expr::VarDecl(Box::new(var_decl::infer(expr)?)),
		Expr::FnDecl(expr) => Expr::FnDecl(Box::new(fn_decl::infer(expr)?)),
		_ => input.clone()
	})
}

pub fn infer_file<'a>(input: &File<'a>) -> Result<File<'a>, String> {
	let mut exprs = Vec::new();

	for expr in &input.exprs {
		exprs.push(infer(expr)?);
	}

	Ok(File { exprs })
}