use nom::{
	IResult, Parser,
	character::complete::{alphanumeric1, alpha1},
	sequence::pair,
	branch::alt,
	multi::many0
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::RESERVED;

#[derive(Debug)]
pub struct Ident<'a> {
	pub value: &'a str
}

impl<'a> Ident<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		pair(
			alt((alpha1, tag("_"))),
			many0(alt((alphanumeric1, tag("_"))))
		)
			.recognize()
			.verify(|value| !RESERVED.contains(value))
			.parse(input)
			.map(|(input, value)| {
				(input, Self { value })
			})
	}
}