use nom::{
	IResult, Parser,
	branch::alt
};

use nom_supreme::{
	error::ErrorTree,
	tag::complete::tag
};

#[derive(Debug)]
pub struct Bool {
	pub value: bool
}

impl Bool {
	pub fn parse(input: &str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			tag("false"),
			tag("true")
		))
			.parse(input)
			.map(|(input, value)| {
				(input, Self { value: value.parse().unwrap() })
			})
	}
}