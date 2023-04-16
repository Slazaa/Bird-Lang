use nom::{
	IResult, Parser,
	sequence::{tuple, delimited},
	multi::separated_list1,
	combinator::opt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{ident::Ident, r#type::Type, block::Block, ws};

#[derive(Debug, Clone)]
pub struct ParamDecl<'a> {
	pub comp: bool,
	pub ident: Ident<'a>,
	pub r#type: Type<'a>
}

impl<'a> ParamDecl<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			opt(tag("comp")).map(|e| e.is_some()),
			ws(Ident::parse),
			ws(Type::parse).preceded_by(tag(":"))
		))
			.parse(input)
			.map(|(input, (comp, ident, r#type))| {
				(input, Self { comp, ident, r#type })
			})
	}
}

#[derive(Debug, Clone)]
pub struct FnDecl<'a> {
	pub inputs:	Vec<ParamDecl<'a>>,
	pub output: Option<Type<'a>>,
	pub body: Block<'a>
}

impl<'a> FnDecl<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			opt(ws(delimited(
				tag("("), separated_list1(tag(","), ws(ParamDecl::parse)), tag(")")
			))).map(|e| if let Some(e) = e { e } else { Vec::new() }),
			opt(ws(Type::parse).preceded_by(tag("->"))),
			ws(Block::parse)
		))
		 	.preceded_by(tag("fn"))
			.parse(input)
			.map(|(input, (inputs, output, body))| {
				(input, Self { inputs, output, body })
			})
	}
}
