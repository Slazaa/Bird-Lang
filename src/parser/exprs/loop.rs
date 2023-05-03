use nom::{
	IResult, Parser,
	sequence::tuple,
	branch::alt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, ws, block::Block};

#[derive(Debug, Clone)]
pub struct Loop<'a> {
	pub cond: Option<Expr<'a>>,
	pub body: Block<'a>
}

impl<'a> Loop<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			ws(Block::parse).map(|x| (None, x)),
			tuple((
				ws(Expr::parse).map(|x| Some(x)),
				ws(Block::parse)
			))
		))
			.preceded_by(tag("loop"))
			.parse(input)
			.map(|(input, (cond, body))| {
				(input, Self { cond, body })
			})
	}
}
