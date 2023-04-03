use nom::{
	IResult, Parser,
	multi::separated_list1
};

use nom_supreme::{
	error::ErrorTree,
	tag::complete::tag
};

use super::ident::Ident;

#[derive(Debug)]
pub struct Path<'a> {
	pub idents: Vec<Ident<'a>>
}

impl<'a> Path<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		separated_list1(tag("."), Ident::parse)
			.parse(input)
			.map(|(input, idents)| {
				(input, Self { idents })
			})
	}
}