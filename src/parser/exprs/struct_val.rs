use nom::{
	IResult, Parser,
	combinator::opt,
	sequence::{tuple, delimited},
	branch::alt,
	multi::separated_list0
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree,
	tag::complete::tag
};

use super::{Expr, ws, ident::Ident, r#type::Type};

#[derive(Debug)]
pub struct FieldVal<'a> {
	pub name: Option<Ident<'a>>,
	pub value: Expr<'a>
}

impl<'a> FieldVal<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			opt(ws(Ident::parse).terminated(tag(":"))),
			ws(Expr::parse)
		))
			.parse(input)
			.map(|(input, (name, value))| {
				(input, Self { name, value })
			})
	}
}

#[derive(Debug)]
pub struct StructVal<'a> {
	pub expr: Option<Type<'a>>,
	pub field_vals: Vec<FieldVal<'a>>
}

impl<'a> StructVal<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			ws(alt((
				Type::parse.map(|e| Some(e)),
				tag(".").map(|_| None)
			))),
			ws(delimited(
				tag("{"), ws(separated_list0(tag(","), ws(FieldVal::parse))), tag("}")
			))
		))
			.parse(input)
			.map(|(input, (expr, field_vals))| {
				(input, Self { expr, field_vals })
			})
	}
}
