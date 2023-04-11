use nom::{
	IResult, Parser,
	character::complete::digit1
};

use nom_supreme::error::ErrorTree;

#[derive(Debug, Clone)]
pub struct Int<'a> {
	pub value: &'a str
}

impl<'a> Int<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		digit1
			.parse(input)
			.map(|(input, value)| {
				(input, Self { value })
			})
	}
}