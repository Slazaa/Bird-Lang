use parse::{LexerBuilder, ParserBuilder, Token, ASTNode, Loc, LexerError, ParserError};
use bird_utils::*;

pub mod patterns;

use crate::patterns::*;

#[derive(Debug, Clone)]
pub enum Node {
	Token(Token),
	// ----------
	AssignExpr(AssignExpr),
	BinExpr(BinExpr),
	Expr(Expr),
	ExternBlock(ExternBlock),
	Func(Func),
	FuncProto(FuncProto),
	IfExpr(IfExpr),
	Item(Item),
	Items(Items),
	Literal(Literal),
	Program(Program),
	Stmt(Stmt),
	Stmts(Stmts),
	Type(Type),
	UnaryExpr(UnaryExpr),
	VarDecl(VarDecl)
}

impl Node {
	fn loc(&self) -> &Loc {
		match self {
			Self::Token(x) => &x.loc,
			
			Self::AssignExpr(x) => &x.loc,
			Self::BinExpr(x) => &x.loc,
			Self::Expr(x) => x.loc(),
			Self::ExternBlock(x) => &x.loc,
			Self::Func(x) => &x.loc,
			Self::FuncProto(x) => &x.loc,
			Self::IfExpr(x) => &x.loc,
			Self::Item(x) => x.loc(),
			Self::Items(x) => &x.loc,
			Self::Literal(x) => &x.loc,
			Self::Program(x) => &x.loc,
			Self::Stmt(x) => x.loc(),
			Self::Stmts(x) => &x.loc,
			Self::Type(x) => &x.loc,
			Self::UnaryExpr(x) => &x.loc,
			Self::VarDecl(x) => &x.loc
		}
	}
}

impl ASTNode for Node {
	fn new_token(token: &Token) -> Self {
		Self::Token(token.to_owned())
	}

	fn token(&self) -> Result<&Token, String> {
		match self {
			Self::Token(token) => Ok(token),
			_ => Err("Node is not a token".to_owned())
		}
	}

	fn is_token(&self) -> bool {
		matches!(self, Self::Token(_))
	}
}

pub fn parse(filename: &str) -> Result<Node, Feedback> {
	let mut lexer_builder = LexerBuilder::new();

	lexer_builder.ignore_rules(&[
		r"(^[ \t\r\n]+)", // Blank spaces
		r"(^#.*)" // Comments
	]).unwrap();

	lexer_builder.add_rules(&[
		// Keywords
		("FUNC",  r"(^func)"),
		("EXT",   r"(^extern)"),
		("IF",    r"(^if)"),
		("IMP",   r"(^import)"),
		("MUT",   r"(^mut)"),
		("VAR",   r"(^var)"),

		// Operators
		("PLUS",  r"(^\+)"),
		("MINUS", r"(^-)"),
		("MULT",  r"(^\*)"),
		("DIV",   r"(^/)"),

		("AMP",   r"(^&)"),

		("EQ",    r"(^=)"),

		// Identifier / Literal
		("BOOL",  r"(^(false|true))"),
		("ID",    r"(^[a-zA-Z_][a-zA-Z0-9_]*)"),
		("FLT",   r"(^\d+\.\d+)"),
		("INT",   r"(^\d+)"),
		("CHR",   r"(^'.')"),
		("STR",   r#"(^".*")"#),

		// Misc
		("COL",   r"(^:)"),
		("LCBR",  r"(^\{)"),
		("LPAR",  r"(^\()"),
		("RCBR",  r"(^\})"),
		("RPAR",  r"(^\))"),
		("SEMI",  r"(^;)")
	]).unwrap();

	let lexer = lexer_builder.build();
/*
	for token in lexer.lex(&input) {
		match token {
			Ok(token) => println!("{:#?}", token),
			Err(e) => {
				println!("{:?}", e);
				break;
			}
		}
	}
*/
	let mut parser_builder = ParserBuilder::<Node, Feedback>::new(&lexer.rules().iter().map(|x| x.name().as_str()).collect::<Vec<&str>>());

	parser_builder.add_patterns(&ASSIGN_PATTERNS).unwrap();
	parser_builder.add_patterns(&BIN_OP_PATTERNS).unwrap();
	parser_builder.add_patterns(&EXPR_PATTERNS).unwrap();
	parser_builder.add_patterns(&EXTERN_BLOCK_PATTERNS).unwrap();
	parser_builder.add_patterns(&FUNC_PROTO_PATTERNS).unwrap();
	parser_builder.add_patterns(&FUNC_PATTERNS).unwrap();
	parser_builder.add_patterns(&IF_EXPR_PATTERNS).unwrap();
	parser_builder.add_patterns(&ITEM_PATTERNS).unwrap();
	parser_builder.add_patterns(&ITEMS_PATTERNS).unwrap();
	parser_builder.add_patterns(&LITERAL_PATTERNS).unwrap();
	parser_builder.add_patterns(&PROGRAM_PATTERNS).unwrap();
	parser_builder.add_patterns(&STMT_PATTERNS).unwrap();
	parser_builder.add_patterns(&STMTS_PATTERNS).unwrap();
	parser_builder.add_patterns(&TYPE_PATTERNS).unwrap();
	parser_builder.add_patterns(&UNARY_OP_PATTERNS).unwrap();
	parser_builder.add_patterns(&VAR_DECL_PATTERNS).unwrap();
	
	let mut parser = parser_builder.build();

	let tokens = match lexer.lex_from_file::<Feedback>(filename) {
		Ok(x) => x,
		Err(e) => {
			return Err(match e {
				LexerError::FileNotFound(filename) => Error::no_file_or_dir(&filename),
				LexerError::InvalidToken(pos) => {
					let loc = Loc { filename: None, start: pos.to_owned(), end: pos };
					Error::invalid_syntax(Some(&loc), "Invalid token")
				}
			})
		}
	};

	match parser.parse(&tokens) {
		Ok(x) => Ok(x),
		Err(e) => {
			Err(match e {
				ParserError::InvalidPatternName(pattern_name) => Error::unspecified(&format!("Invalid pattern name: '{}'", pattern_name)),
				ParserError::NotMatching(_) => Error::invalid_syntax(None, "Could not create program"),
				ParserError::PatternFunc(feedback) => feedback,
				ParserError::TokenRemaining => Error::unspecified("Unevaluated tokens remaining"),
				ParserError::UnknownElem(elem) => panic!("Unknown element: '{}'", elem)
			})
		}
	}
}