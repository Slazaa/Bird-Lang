use nom::{
	IResult, Parser,
	sequence::tuple
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, ws, block::Block};

#[derive(Debug)]
pub struct While<'a> {
	pub cond: Expr<'a>,
	pub body: Block<'a>
}

impl<'a> While<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(Expr::parse).preceded_by(tag("while")),
			ws(Block::parse)
		))
			.parse(input)
			.map(|(input, (cond, body))| {
				(input, Self { cond, body })
			})
	}
}
