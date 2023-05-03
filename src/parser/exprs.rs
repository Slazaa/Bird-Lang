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
		num::Num,
		string::String
	},
	assign::Assign,
	block::Block,
	box_decl::BoxDecl,
	enum_decl::EnumDecl,
	fn_call::FnCall,
	fn_decl::FnDecl,
	ident::Ident,
	r#if::If,
	r#loop::Loop,
	path::Path,
	r#return::Return,
	struct_decl::StructDecl,
	struct_val::StructVal,
	r#type::Type,
	vis::Vis,
};

pub mod assign;
pub mod block;
pub mod box_decl;
pub mod enum_decl;
pub mod file;
pub mod fn_call;
pub mod fn_decl;
pub mod ident;
pub mod r#if;
pub mod literals;
pub mod r#loop;
pub mod path;
pub mod r#return;
pub mod struct_decl;
pub mod struct_val;
pub mod r#type;
pub mod vis;

pub const RESERVED: [&str; 13] = [
	"box"   , "else"  , "enum"  ,
	"false" , "fn"    , "if"    ,
	"loop"  , "match" , "mut"   ,
	"pub"   , "return", "struct",
	"true"  ,
];


pub fn ignore_comment(input: &str) -> IResult<&str, (), ErrorTree<&str>> {
	not_line_ending.preceded_by(tag("#"))
		.parse(input)
		.map(|(input, _)| (input, ()))
}

pub fn ws<'a, O, F>(f: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, ErrorTree<&'a str>>
where
	F: Parser<&'a str, O, ErrorTree<&'a str>>
{
	fn parser(input: &str) -> IResult<&str, (), ErrorTree<&str>> {
		many0(alt((multispace1.map(|_| ()), ignore_comment)))
			.parse(input)
			.map(|(input, _)| (input, ()))
	}

	delimited(parser, f, parser)
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
	// Literals
	Bool(Bool),
	Char(Char<'a>),
	Num(Num<'a>),
	String(String<'a>),

	// ----------
	Assign(Box<Assign<'a>>),
	Block(Box<Block<'a>>),
	BoxDecl(Box<BoxDecl<'a>>),
	EnumDecl(EnumDecl<'a>),
	FnCall(Box<FnCall<'a>>),
	FnDecl(Box<FnDecl<'a>>),
	Ident(Ident<'a>),
	If(Box<If<'a>>),
	Loop(Box<Loop<'a>>),
	Path(Path<'a>),
	Return(Box<Return<'a>>),
	StructDecl(StructDecl<'a>),
	StructVal(Box<StructVal<'a>>),
	Type(Box<Type<'a>>),
}

impl<'a> Expr<'a> {
	pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
		alt((
			// ----------
			Assign::parse.map(|x| Expr::Assign(Box::new(x))),
			Block::parse.map(|x| Expr::Block(Box::new(x))),
			BoxDecl::parse.map(|x| Expr::BoxDecl(Box::new(x))),
			EnumDecl::parse.map(|x| Expr::EnumDecl(x)),
			If::parse.map(|x| Expr::If(Box::new(x))),
			Loop::parse.map(|x| Expr::Loop(Box::new(x))),
			Path::parse_fn_call.map(|x| Expr::Path(x)),
			Return::parse.map(|x| Expr::Return(Box::new(x))),
			FnCall::parse.map(|x| Expr::FnCall(Box::new(x))),
			FnDecl::parse.map(|x| Expr::FnDecl(Box::new(x))),
			StructDecl::parse.map(|x| Expr::StructDecl(x)),
			StructVal::parse.map(|x| Expr::StructVal(Box::new(x))),

			Path::parse_ident.map(|x| Expr::Path(x)),

			// Literals
			Bool::parse.map(|x| Expr::Bool(x)),
			Char::parse.map(|x| Expr::Char(x)),
			Num::parse.map(|x| Expr::Num(x)),
			String::parse.map(|x| Expr::String(x)),
		))(input)
	}
}