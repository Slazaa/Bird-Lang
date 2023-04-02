use nom::{
	IResult, Parser, InputTakeAtPosition, AsChar,
	error::ParseError,
	sequence::delimited,
	character::complete::multispace0,
	combinator::map,
	branch::alt
};

use nom_supreme::error::ErrorTree;

use self::{
	literals::{
		bool::Bool,
		char::Char,
		float::Float,
		int::Int,
		string::String
	},
	block::Block,
	box_decl::BoxDecl,
	fn_decl::FnDecl,
	ident::Ident
};

pub mod block;
pub mod box_decl;
pub mod file;
pub mod fn_decl;
pub mod ident;
pub mod literals;

pub const RESERVED: [&str; 11] = [
    "box" , "else"  , "false",
	"fn"  , "if"    , "mut"  ,
	"pub" , "struct", "trait",
	"true", "while"
];

pub fn ws<I, O, E>(parser: impl Parser<I, O, E>) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition, <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>
{
    delimited(multispace0, parser, multispace0)
}

#[derive(Debug)]
pub enum Vis {
	Private,
	Public
}

#[derive(Debug)]
pub enum Expr<'a> {
	// Literals
	Bool(Bool),
	Char(Char<'a>),
	Float(Float<'a>),
	Int(Int<'a>),
	String(String<'a>),

	// ----------
	Block(Box<Block<'a>>),
	BoxDecl(Box<BoxDecl<'a>>),
	FnDecl(Box<FnDecl<'a>>),
	Ident(Ident<'a>)
}

impl<'a> Expr<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			// ----------
			map(Block::parse, |x| Expr::Block(Box::new(x))),
			map(BoxDecl::parse, |x| Expr::BoxDecl(Box::new(x))),
			map(FnDecl::parse, |x| Expr::FnDecl(Box::new(x))),
			map(Ident::parse, |x| Expr::Ident(x)),

			// Literals
			map(Bool::parse, |x| Expr::Bool(x)),
			map(Char::parse, |x| Expr::Char(x)),
			map(Float::parse, |x| Expr::Float(x)),
			map(Int::parse, |x| Expr::Int(x)),
			map(String::parse, |x| Expr::String(x))
		))(input)
	}
}