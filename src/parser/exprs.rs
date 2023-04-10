use nom::{
	IResult, Parser,
	character::complete::multispace1,
	branch::alt,
	bytes::complete::tag,
	multi::many0,
	character::complete::not_line_ending,
	sequence::delimited
};

use nom_supreme::{
	ParserExt,
	error::ErrorTree
};

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
	enum_decl::EnumDecl,
	file::File,
	fn_call::FnCall,
	fn_decl::FnDecl,
	ident::Ident,
	r#impl::Impl,
	r#if::If,
	path::Path,
	struct_decl::StructDecl,
	struct_val::StructVal,
	r#type::Type,
	vis::Vis,
	r#while::While
};

pub mod block;
pub mod box_decl;
pub mod enum_decl;
pub mod file;
pub mod fn_call;
pub mod fn_decl;
pub mod ident;
pub mod r#if;
pub mod r#impl;
pub mod literals;
pub mod path;
pub mod struct_decl;
pub mod struct_val;
pub mod r#type;
pub mod vis;
pub mod r#while;

pub const RESERVED: [&str; 14] = [
	"box"  , "else"  , "enum" ,
	"false", "fn"    , "if"   ,
	"impl" , "match" , "mut"  ,
	"pub"  , "struct", "trait",
	"true" , "while"
];


pub fn ignore_comment(input: &str) -> IResult<&str, (), ErrorTree<&str>> {
	many0(not_line_ending.preceded_by(tag("#")))
		.parse(input)
		.map(|(input, _)| (input, ()))
}

pub fn ws<'a, O, F>(f: F) -> impl FnMut(&str) -> IResult<&str, O, ErrorTree<&str>>
where
	F: Parser<&'a str, O, ErrorTree<&'a str>>
{
	fn parser(input: &str) -> IResult<&str, (), ErrorTree<&str>> {
		many0(alt((multispace1, ignore_comment)))
			.parse(input)
			.map(|(input, _)| (input, ()))
	}

	delimited(parser, f, parser)
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
	EnumDecl(EnumDecl<'a>),
	File(File<'a>),
	FnCall(Box<FnCall<'a>>),
	FnDecl(Box<FnDecl<'a>>),
	Ident(Ident<'a>),
	If(Box<If<'a>>),
	Impl(Box<Impl<'a>>),
	Path(Path<'a>),
	StructDecl(StructDecl<'a>),
	StructVal(Box<StructVal<'a>>),
	Type(Box<Type<'a>>),
	While(Box<While<'a>>)
}

impl<'a> Expr<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			// ----------
			Block::parse.map(|x| Expr::Block(Box::new(x))),
			BoxDecl::parse.map(|x| Expr::BoxDecl(Box::new(x))),
			EnumDecl::parse.map(|x| Expr::EnumDecl(x)),
			If::parse.map(|x| Expr::If(Box::new(x))),
			Impl::parse.map(|x| Expr::Impl(Box::new(x))),
			Path::parse_fn_call.map(|x| Expr::Path(x)),
			FnCall::parse.map(|x| Expr::FnCall(Box::new(x))),
			FnDecl::parse.map(|x| Expr::FnDecl(Box::new(x))),
			StructDecl::parse.map(|x| Expr::StructDecl(x)),
			StructVal::parse.map(|x| Expr::StructVal(Box::new(x))),
			While::parse.map(|x| Expr::While(Box::new(x))),

			Path::parse_ident.map(|x| Expr::Path(x)),

			// Literals
			Bool::parse.map(|x| Expr::Bool(x)),
			Char::parse.map(|x| Expr::Char(x)),
			Float::parse.map(|x| Expr::Float(x)),
			Int::parse.map(|x| Expr::Int(x)),
			String::parse.map(|x| Expr::String(x))
		))(input)
	}
}
