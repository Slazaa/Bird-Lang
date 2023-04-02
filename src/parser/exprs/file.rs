use nom::{
	IResult, Parser,
	combinator::{all_consuming, opt},
	multi::many0,
	sequence::terminated
};

use nom_supreme::{
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, ws};

#[derive(Debug)]
pub struct File {
	pub exprs: Vec<Expr>
}

impl File {
	pub fn parse(input: &str) -> IResult<&str, Self, ErrorTree<&str>> {
		all_consuming(ws(many0(terminated(
			ws(Expr::parse),
			opt(tag(";"))
		))))
			.parse(input)
			.map(|(input, exprs)| {
				(input, Self { exprs })
			})
	}
}