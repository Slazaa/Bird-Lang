use crate::{parser::exprs::{r#impl::Impl, Expr, box_decl::BoxDecl, r#type::Type, ident::Ident}, transpiler::c::ident};

fn type_to_ident(input: &Type) -> String {
	todo!()
}

pub fn transpile(input: &Impl) -> String {
	let boxes: Vec<BoxDecl> = input.body.exprs.iter()
		.map(|expr| match expr {
			Expr::BoxDecl(expr) => BoxDecl {
				ident: Ident { value: &(type_to_ident(expr.r#type.as_ref().unwrap()) + &ident::transpile(&expr.ident)) },
				..*expr.clone()
			},
			_ => panic!("Expected BoxDecl")
		})
		.collect();

	todo!()
}