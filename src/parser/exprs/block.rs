use nom::{
	IResult, Parser,
	sequence::delimited,
	multi::many0
};

use nom_supreme::{
	tag::complete::tag,
	error::ErrorTree, ParserExt
};

use super::{Expr, ws};

#[derive(Debug)]
pub struct Block<'a> {
	pub exprs: Vec<Expr<'a>>
}

impl<'a> Block<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		delimited(
			tag("{"), ws(many0(ws(Expr::parse.terminated(tag(";"))))), tag("}")
		)
			.parse(input)
			.map(|(input, exprs)| {
				(input, Self { exprs })
			})
	}
}
