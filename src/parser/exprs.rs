use nom::{
	IResult, Parser, InputTakeAtPosition, AsChar,
	error::ParseError,
	sequence::delimited,
	character::complete::multispace0,
	combinator::map
};

use nom_supreme::error::ErrorTree;

use literals::bool_::Bool;

pub mod file;
pub mod literals;

pub fn ws<I, O, E>(parser: impl Parser<I, O, E>) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition, <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>
{
    delimited(multispace0, parser, multispace0)
}

#[derive(Debug)]
pub enum Expr {
	Bool(Bool)
}

impl Expr {
	pub fn parse(input: &str) -> IResult<&str, Self, ErrorTree<&str>> {
		map(Bool::parse, |x| Expr::Bool(x))(input)
	}
}