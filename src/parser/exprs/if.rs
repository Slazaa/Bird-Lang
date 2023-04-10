use nom::{
	IResult, Parser,
	sequence::tuple,
	combinator::opt,
	branch::alt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, ws, block::Block};

#[derive(Debug)]
pub enum IfBranch<'a> {
	ElseIf(Box<If<'a>>),
	Else(Block<'a>)
}

#[derive(Debug)]
pub struct If<'a> {
	pub cond: Expr<'a>,
	pub body: Block<'a>,
	pub branch: Option<IfBranch<'a>>
}

impl<'a> If<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(Expr::parse).preceded_by(tag("if")),
			ws(Block::parse),
			opt(alt((
				ws(Self::parse).map(|e| IfBranch::ElseIf(Box::new(e))),
				ws(Block::parse).map(|e| IfBranch::Else(e))
			)).preceded_by(tag("else")))
		))
			.parse(input)
			.map(|(input, (cond, body, branch))| {
				(input, Self { cond, body, branch })
			})
	}
}
