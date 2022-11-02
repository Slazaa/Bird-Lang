use std::fs;

use parse::{LexerBuilder, ParserBuilder, Token, ASTNode};
use bird_utils::feedback::*;

mod patterns;

use crate::patterns::*;

#[derive(Debug, Clone)]
pub enum Item {
	Func(Func),
	FuncProto(FuncProto),
	VarDecl(VarDecl)
}

#[derive(Debug, Clone)]
pub struct Func {
	pub id: String,
	pub stmts: Stmts
}

#[derive(Debug, Clone)]
pub struct FuncProto {
	pub id: String
}

#[derive(Debug, Clone)]
pub enum Stmt {
	Expr(Expr),
	Item(Item)
}

#[derive(Debug, Clone)]
pub struct Stmts {
	pub stmts: Vec<Stmt>
}

#[derive(Debug, Clone)]
pub struct BinExpr {
	pub left: Expr,
	pub op: String,
	pub right: Expr
}

#[derive(Debug, Clone)]
pub enum Node {
	Token(Token),
	// ----------
	Item(Item),
	Func(Func),
	FuncProto(FuncProto),
	Stmt(Stmt),
	Stmts(Stmts),
	Literal(Literal),
	Expr(Expr),
	Program(Stmts),
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

fn func(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name() == "ID" => token.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	let stmts = match &nodes[3] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[4]))
	};

	Ok(Node::Func(Func { id, stmts }))
}

fn func_proto(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name() == "ID" => token.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	Ok(Node::FuncProto(FuncProto { id }))
}

fn item(nodes: &[Node]) -> Result<Node, String> {
	match &nodes[0] {
		Node::Func(x) => Ok(Node::Item(Item::Func(x.to_owned()))),
		Node::FuncProto(x) => Ok(Node::Item(Item::FuncProto(x.to_owned()))),
		Node::VarDecl(x) => Ok(Node::Item(Item::VarDecl(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'item'", nodes[0]))
	}
}

fn program(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Program(Stmts { stmts: vec![] }));
	}

	match &nodes[0] {
		Node::Stmts(x) => Ok(Node::Program(x.to_owned())),
		_ => Err(format!("Invalid node '{:?}' in 'program'", nodes[0]))
	}
}

fn stmt(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Stmt(match nodes[0].to_owned() {
		Node::Expr(x) => Stmt::Expr(x),
		Node::Item(x) => Stmt::Item(x),
		_ => return Err(format!("Invalid node '{:?}' in 'stmt'", nodes[0]))
	}))
}

fn stmts(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Stmts(Stmts { stmts: vec![] }));
	}

	let node_stmt = match &nodes[0] {
		Node::Stmt(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'stmts'", nodes[0]))
	};

	let node_stmts = match nodes.get(1) {
		Some(Node::Stmts(x)) => x.stmts.clone(),
		_ => Vec::new()
	};

	let mut stmts_vec = vec![node_stmt];
	stmts_vec.extend(node_stmts);

	Ok(Node::Stmts(Stmts { stmts: stmts_vec }))
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
		r"(^[ \t\r\n]+)"
	]).unwrap();

	lexer_builder.add_rules(&[
		// Keywords
		("FUNC",  r"(^func)"),
		("VAR",   r"(^var)"),

		// Operators
		("PLUS",  r"(^\+)"),
		("MINUS", r"(^-)"),
		("MULT",  r"(^\*)"),
		("DIV",   r"(^/)"),
		("EQ",    r"(^=)"),

		// Identifier / Literal
		("ID",    r"(^[a-zA-Z_][a-zA-Z0-9_]*)"),
		("FLOAT", r"(^\d+\.\d+)"),
		("INT",   r"(^\d+)"),
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

	parser_builder.add_patterns(&[
		("func",       "FUNC ID LCBR stmts RCBR", func),
		("func_proto", "FUNC ID", func_proto),
		("item",       "func", item),
		("item",       "func_proto", item),
		("item",       "var_decl", item),
		("program",    "stmts", program),
		("program",    "", program),
		("stmt",       "item", stmt),
		("stmts",      "stmt stmts", stmts),
		("stmts",      "stmt", stmts),
		("stmts",      "", stmts),
	]).unwrap();

	parser_builder.add_patterns(&EXPR_PATTERNS).unwrap();
	parser_builder.add_patterns(&LITERAL_PATTERNS).unwrap();
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