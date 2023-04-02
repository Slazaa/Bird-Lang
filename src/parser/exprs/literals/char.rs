use nom::{
	IResult, Parser,
	bytes::complete::take_while_m_n,
	branch::alt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

#[derive(Debug)]
pub struct Char<'a> {
	pub value: &'a str
}

impl<'a> Char<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			tag("\\n"), tag("\\t"),
			take_while_m_n(1, 1, |_| true)
		))
			.delimited_by(tag("'"))
			.parse(input)
			.map(|(input, value)| {
				(input, Self { value })
			})
	}
}