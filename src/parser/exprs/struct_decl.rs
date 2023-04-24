use nom::{
	IResult, Parser,
	sequence::{tuple, delimited},
	multi::separated_list1,
	branch::alt
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{ident::Ident, r#type::Type, ws};

#[derive(Debug, Clone)]
pub struct Field<'a> {
	pub ident: Ident<'a>,
	pub r#type: Type<'a>
}

impl<'a> Field<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(Ident::parse).terminated(tag(":")),
			ws(Type::parse)
		))
			.parse(input)
			.map(|(input, (ident, r#type))| {
				(input, Self { ident, r#type })
			})
	}
}

#[derive(Debug, Clone)]
pub struct StructDecl<'a> {
	pub ident: Ident<'a>,
	pub fields: Option<Vec<Field<'a>>>
}

impl<'a> StructDecl<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(Ident::parse),
			ws(alt((
				delimited(tag("{"), ws(separated_list1(tag(","), ws(Field::parse))), tag("}")).map(|x| Some(x)),
				tag(";").map(|_| None)
			)))
		))
			.preceded_by(tag("struct"))
			.parse(input)
			.map(|(input, (ident, fields))| {
				(input, Self { ident, fields })
			})
	}
}
