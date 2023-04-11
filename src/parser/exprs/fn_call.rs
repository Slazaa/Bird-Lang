use nom::{
	IResult, Parser,
	combinator::recognize,
	sequence::delimited,
	bytes::complete::take_until,
	multi::separated_list0,
	branch::alt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag 
};

use crate::parser::exprs::ws;

use super::{Expr, block::Block, path::Path};

#[derive(Debug, Clone)]
pub struct FnCall<'a> {
	pub expr: Expr<'a>,
	pub inputs: Vec<Expr<'a>>
}

impl<'a> FnCall<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		fn last_paren_group<'a>(input: &'a str, original: &'a str, offset: usize) -> IResult<&'a str, usize, ErrorTree<&'a str>> {
			let (next_groups, group) = recognize(delimited(
					tag("("), take_until(")"), tag(")")
					))(input)?;

			match last_paren_group(next_groups, original, offset + group.len()) {
				Ok(x) => Ok(x),
				Err(_) => Ok((input, offset))
			}
		}

		let (groups, groups_prefix) = take_until("(")(input)?;
		let (_, last_group_offset) = last_paren_group(groups, groups, 0)?;
		let (input, expr) = (
			&input[groups_prefix.len() + last_group_offset..],
			&input[..groups_prefix.len() + last_group_offset]
			);

		let (input, inputs) = ws(delimited(tag("("),
		ws(separated_list0(
				tag(","), ws(Expr::parse))),
				tag(")")
				))(input)?;

		ws(alt((
			Block::parse.map(|e| Expr::Block(Box::new(e))),
			FnCall::parse.map(|e| Expr::FnCall(Box::new(e))),
			Path::parse.map(|e| Expr::Path(e))
		)))
			.all_consuming()
			.parse(expr)
			.map(|(_, expr)| {
				(input, Self { expr, inputs })
			})
	}
}
