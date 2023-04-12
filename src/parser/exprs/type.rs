use nom::{
	IResult, Parser,
	branch::alt,
	sequence::tuple,
	combinator::opt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, block::Block, enum_decl::EnumDecl, fn_call::FnCall, struct_decl::StructDecl, path::Path, ws};

#[derive(Debug, Clone)]
pub enum PtrKind {
	Const,
	Mutable
}

#[derive(Debug, Clone)]
pub struct Type<'a> {
	pub ptr_kind: Option<PtrKind>,
	pub value: Expr<'a>
}

impl<'a> Type<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(opt(alt((
				tag("*mut").map(|_| PtrKind::Mutable),
				tag("*").map(|_| PtrKind::Const)
			)))),
			ws(alt((
				Block::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::Block(Box::new(e)))),
				EnumDecl::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::EnumDecl(e))),
				FnCall::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::FnCall(Box::new(e)))),
				Path::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::Path(e))),
				StructDecl::parse.map_res::<_, _, ErrorTree<&str>>(|e| Ok(Expr::StructDecl(e)))
			)))
		))
			.parse(input)
			.map(|(input, (ptr_kind, value))| {
				(input, Self { ptr_kind, value })
			})
	}
}