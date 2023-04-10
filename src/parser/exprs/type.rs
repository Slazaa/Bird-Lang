use nom::{
	IResult, Parser,
	branch::alt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree
};

use super::{Expr, block::Block, enum_decl::EnumDecl, fn_call::FnCall, struct_decl::StructDecl, path::Path};

#[derive(Debug)]
pub struct Type<'a> {
	pub value: Expr<'a>
}

impl<'a> Type<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			Block::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::Block(Box::new(e)))),
			EnumDecl::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::EnumDecl(e))),
			FnCall::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::FnCall(Box::new(e)))),
			Path::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::Path(e))),
			StructDecl::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::StructDecl(e)))
		))
			.parse(input)
			.map(|(input, value)| {
				(input, Self { value })
			})
	}
}
