use nom::{
	IResult, Parser,
	multi::separated_list1,
	branch::alt,
	sequence::pair
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, ident::Ident, fn_call::FnCall};

#[derive(Debug, Clone)]
pub struct Path<'a> {
	pub exprs: Vec<Expr<'a>>
}

impl<'a> Path<'a> {
	fn parse_expr(input: &'a str) -> IResult<&str, Expr<'a>, ErrorTree<&str>> {
		alt((
			FnCall::parse.map(|x| Expr::FnCall(Box::new(x))),
			Ident::parse.map(|x| Expr::Ident(x))
		))(input)
	}

	pub fn parse_ident(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		separated_list1(tag("."), Ident::parse.map(|x| Expr::Ident(x)))
			.parse(input)
			.map(|(input, exprs)| {
				(input, Self { exprs })
			})
	}

	pub fn parse_fn_call(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		pair(
			FnCall::parse.map(|x| Expr::FnCall(Box::new(x))).terminated(tag(".")),
			separated_list1(tag("."), Self::parse_expr)
		).map(|(head, mut tail)| {
			tail.insert(0, head);
			tail
		})
			.parse(input)
			.map(|(input, exprs)| {
				(input, Self { exprs })
			})
	}

	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			Self::parse_ident,
			Self::parse_fn_call
		))(input)
	}
}
