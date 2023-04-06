use nom::{
	IResult, Parser,
	sequence::tuple,
	combinator::opt
};

use nom_supreme::{
	error::ErrorTree,
	tag::complete::tag, ParserExt
};

use super::{Expr, Vis, ident::Ident, r#type::Type, ws};

#[derive(Debug)]
pub struct BoxDecl<'a> {
	pub vis: Option<Vis>,
	pub r#mut: bool,
	pub ident: Ident<'a>,
	pub r#type: Option<Type<'a>>,
	pub value: Option<Expr<'a>>
}

impl<'a> BoxDecl<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		tuple((
			opt(ws(tag("mut"))).map(|e| e.is_some()),
			ws(Ident::parse),
			opt(ws(Type::parse).preceded_by(tag(":"))),
			opt(ws(Expr::parse).preceded_by(tag("=")))
		))
			.preceded_by(tag("box"))
			.parse(input)
			.map(|(input, (r#mut, ident, r#type, value))| {
				(input, Self { vis: None, r#mut, ident, r#type, value })
			})
	}
}
