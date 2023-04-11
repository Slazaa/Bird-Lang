use nom::{
	IResult, Parser,
	character::complete::digit1
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag 
};

#[derive(Debug, Clone)]
pub struct Float<'a> {
	pub value: &'a str
}

impl<'a> Float<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tag(".")
			.delimited_by(digit1)
			.recognize()
			.parse(input)
			.map(|(input, value)| {
				(input, Self { value })
			})
	}
}