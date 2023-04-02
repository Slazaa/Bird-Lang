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
	ident::Ident,
	proc_decl::ProcDecl
};

pub mod block;
pub mod box_decl;
pub mod file;
pub mod ident;
pub mod literals;
pub mod proc_decl;

pub const RESERVED: [&str; 12] = [
    "box"   , "else", "false" ,
    "if"    , "impl", "import",
    "mut"   , "proc", "pub"   ,
    "struct", "true", "while"
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
	Ident(Ident<'a>),
	ProcDecl(Box<ProcDecl<'a>>)
}

impl<'a> Expr<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			map(Block::parse, |x| Expr::Block(Box::new(x))),
			map(BoxDecl::parse, |x| Expr::BoxDecl(Box::new(x))),
			map(Ident::parse, |x| Expr::Ident(x)),
			map(ProcDecl::parse, |x| Expr::ProcDecl(Box::new(x))),

			map(Bool::parse, |x| Expr::Bool(x)),
			map(Char::parse, |x| Expr::Char(x)),
			map(Float::parse, |x| Expr::Float(x)),
			map(Int::parse, |x| Expr::Int(x)),
			map(String::parse, |x| Expr::String(x))
		))(input)
	}
}