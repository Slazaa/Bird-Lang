use nom::{
	IResult, Parser,
	bytes::complete::take_until
};

use nom_supreme::{
	error::ErrorTree,
	tag::complete::tag, ParserExt
};

#[derive(Debug)]
pub struct String<'a> {
	pub value: &'a str
}

impl<'a> String<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		take_until("\"")
			.delimited_by(tag("\""))
			.parse(input)
			.map(|(input, value)| {
				(input, Self { value })
			})
	}
}