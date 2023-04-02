use nom::IResult;
use nom_supreme::error::ErrorTree;

#[derive(Debug)]
pub struct Int<'a> {
	pub value: &'a str
}

impl<'a> Int<'a> {
	pub fn parse(input: &str) -> IResult<&str, Self, ErrorTree<&str>> {
		
	}
}