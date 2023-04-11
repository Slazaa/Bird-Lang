use nom::{
	IResult, Parser,
	multi::many0
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, ws};

#[derive(Debug, Clone)]
pub struct File<'a> {
	pub exprs: Vec<Expr<'a>>
}

impl<'a> File<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		ws(many0(
			ws(Expr::parse).terminated(tag(";"))
		))
			.all_consuming()
			.parse(input)
			.map(|(input, exprs)| {
				(input, Self { exprs })
			})
	}
}
