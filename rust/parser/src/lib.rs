use std::fs;

use parse::{LexerBuilder, ParserBuilder, Token, ASTNode};
use bird_utils::feedback::*;

mod patterns;

use crate::patterns::*;

#[derive(Debug, Clone)]
pub enum Node {
	Token(Token),
	// ----------
	Expr(Expr),
	Func(Func),
	FuncProto(FuncProto),
	IfExpr(IfExpr),
	Item(Item),
	Literal(Literal),
	Program(Stmts),
	Stmt(Stmt),
	Stmts(Stmts),
	VarDecl(VarDecl)
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
	let input = match fs::read_to_string(filename) {
		Ok(x) => x,
		Err(_) => {
			println!("Invalid filename '{}'", filename);
			return Err(Error::no_file_or_dir(filename));
		}
	};

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
		("VAR",   r"(^var)"),

		// Operators
		("PLUS",  r"(^\+)"),
		("MINUS", r"(^-)"),
		("MULT",  r"(^\*)"),
		("DIV",   r"(^/)"),
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
	let mut parser_builder = ParserBuilder::<Node>::new(&lexer.rules().iter().map(|x| x.name().as_str()).collect::<Vec<&str>>());

	parser_builder.add_patterns(&EXPR_PATTERNS).unwrap();
	parser_builder.add_patterns(&FUNC_PROTO_PATTERNS).unwrap();
	parser_builder.add_patterns(&FUNC_PATTERNS).unwrap();
	parser_builder.add_patterns(&IF_EXPR_PATTERNS).unwrap();
	parser_builder.add_patterns(&ITEM_PATTERNS).unwrap();
	parser_builder.add_patterns(&LITERAL_PATTERNS).unwrap();
	parser_builder.add_patterns(&PROGRAM_PATTERNS).unwrap();
	parser_builder.add_patterns(&STMT_PATTERNS).unwrap();
	parser_builder.add_patterns(&STMTS_PATTERNS).unwrap();
	parser_builder.add_patterns(&VAR_DECL_PATTERNS).unwrap();
	
	let mut parser = parser_builder.build();

	match parser.parse(lexer.lex(&input)) {
		Ok(x) => Ok(x),
		Err((e, pos)) => {
			println!("{:?} at {}", e, pos);
			return Err(Error::invalid_syntax(None, &format!("{:?}", e)));
		}
	}
}