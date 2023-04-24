
use nom::{
	IResult, Parser,
	character::complete::digit1,
	combinator::opt,
	sequence::tuple
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag 
};

#[derive(Debug, Clone)]
pub struct Num<'a> {
	pub value: &'a str
}

impl<'a> Num<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			digit1,
			opt(
				tag(".")
				.terminated(digit1)
			)
		))
			.recognize()
			.parse(input)
			.map(|(input, value)| {
				(input, Self { value })
			})
	}
}