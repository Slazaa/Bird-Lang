use std::ops::Deref;

use crate::parser::exprs::{file::File, Expr, r#type::Type, path::Path, ident::Ident, fn_call::FnCall, struct_val::{StructVal, FieldVal}};

pub mod block;
pub mod box_decl;
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
		Expr::FnDecl(expr) => {
			let input_types: Vec<&Type> = expr.inputs.iter()
				.map(|x| x.r#type.as_ref().unwrap())
				.collect();

			let input_field_vals: Vec<FieldVal> = input_types.iter()
				.map(|x| FieldVal { name: None, value: Expr::Type(Box::new(x.deref().clone())) })
				.collect();

			let ret_type = if let Some(output) = &expr.output {
				output.clone()
			} else {
				type_from("void")
			};

			Type {
				value: Expr::FnCall(Box::new(FnCall {
					expr: Expr::Path(Path { exprs: vec![Expr::Ident(Ident { value: "Proc" })] }),
					inputs: vec![
						Expr::StructVal(Box::new(StructVal {
							expr: None,
							field_vals: input_field_vals
						})),
						Expr::Type(Box::new(ret_type))
					],
				})),
				ptr_kind: None
			}
		}

		_ => todo!("{:?}", input)
	}
}

pub fn infer<'a>(input: &Expr<'a>) -> Result<Expr<'a>, String> {
	Ok(match &input {
		Expr::Block(expr) => Expr::Block(Box::new(block::infer(expr)?)),
		Expr::BoxDecl(expr) if expr.r#type.is_none() && expr.value.is_some() => Expr::BoxDecl(Box::new(box_decl::infer(expr)?)),
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