use nom::{
	IResult, Parser,
	sequence::{tuple, delimited},
	multi::separated_list0,
	combinator::opt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{ident::Ident, Expr, ws};

#[derive(Debug)]
pub struct ParamDecl<'a> {
	pub ident: Ident<'a>,
	pub r#type: Option<Expr<'a>>
}

impl<'a> ParamDecl<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(Ident::parse),
			opt(ws(Expr::parse).preceded_by(tag(":")))
		))
			.parse(input)
			.map(|(input, (ident, r#type))| {
				(input, Self { ident, r#type })
			})
	}
}

#[derive(Debug)]
pub struct ProcDecl<'a> {
	pub inputs:	Vec<ParamDecl<'a>>,
	pub output: Option<Expr<'a>>,
	pub body: Expr<'a>
}

impl<'a> ProcDecl<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(delimited(
				tag("("), separated_list0(tag(","), ws(ParamDecl::parse)), tag(")")
			)),
			opt(ws(Expr::parse).preceded_by(tag("->"))),
			ws(Expr::parse)
		))
		 	.preceded_by(tag("proc"))
			.parse(input)
			.map(|(input, (inputs, output, body))| {
				(input, Self { inputs, output, body })
			})
	}
}