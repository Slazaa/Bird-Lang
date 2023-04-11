use nom::{
	IResult, Parser,
	sequence::{tuple, delimited},
	combinator::opt,
	multi::separated_list1
};

use nom_supreme::{
	error::ErrorTree,
	tag::complete::tag, ParserExt
};

use super::{ident::Ident, ws, struct_decl::Field};

#[derive(Debug, Clone)]
pub struct EnumVal<'a> {
	pub ident: Ident<'a>,
	pub fields: Option<Vec<Field<'a>>>
}

impl<'a> EnumVal<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(Ident::parse),
			opt(ws(delimited(
				tag("{"), ws(separated_list1(tag(","), ws(Field::parse))), tag("}")
			)))
		))
			.parse(input)
			.map(|(input, (ident, fields))| {
				(input, Self { ident, fields })
			})
	}
}

#[derive(Debug, Clone)]
pub struct EnumDecl<'a> {
	pub values: Vec<EnumVal<'a>>
}

impl<'a> EnumDecl<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		ws(delimited(
			tag("{"), ws(separated_list1(tag(","), ws(EnumVal::parse))), tag("}")
		))
			.preceded_by(tag("enum"))
			.parse(input)
			.map(|(input, values)| {
				(input, Self { values })
			})
	}
}