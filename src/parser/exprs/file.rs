use nom::{
	IResult, Parser,
	multi::many0, branch::alt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, ws, fn_decl::FnDecl, struct_decl::StructDecl, enum_decl::EnumDecl};

#[derive(Debug, Clone)]
pub struct File<'a> {
	pub exprs: Vec<Expr<'a>>
}

impl<'a> File<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		ws(many0(
			ws(alt((
				Expr::parse.terminated(tag(";")),
				EnumDecl::parse.map(|x| Expr::EnumDecl(x)),
				FnDecl::parse.map(|x| Expr::FnDecl(Box::new(x))),
				StructDecl::parse.map(|x| Expr::StructDecl(x)),
			)))
		))
			.all_consuming()
			.parse(input)
			.map(|(input, exprs)| {
				(input, Self { exprs })
			})
	}
}
