use nom::{
	IResult, Parser,
	sequence::delimited,
	multi::many0,
	branch::alt
};

use nom_supreme::{
	tag::complete::tag,
	error::ErrorTree, ParserExt
};

use super::{Expr, ws, r#if::If, r#loop::Loop};

#[derive(Debug, Clone)]
pub struct Block<'a> {
	pub exprs: Vec<Expr<'a>>
}

impl<'a> Block<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		delimited(
			tag("{"), ws(many0(ws(alt((
				Expr::parse.terminated(tag(";")),
				Block::parse.map(|x| Expr::Block(Box::new(x))),
				If::parse.map(|x| Expr::If(Box::new(x))),
				Loop::parse.map(|x| Expr::Loop(Box::new(x))),
			))))), tag("}")
		)
			.parse(input)
			.map(|(input, exprs)| {
				(input, Self { exprs })
			})
	}
}
