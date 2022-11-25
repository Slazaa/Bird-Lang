use parse::{PatternFunc, Loc, ASTNode};

use crate::Node;

use super::Stmts;

#[derive(Debug, Clone)]
pub struct Func {
	pub id: String,
	pub stmts: Stmts,
	pub loc: Loc
}

pub static FUNC_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 2] = [
	("func", "FUNC ID LCBR stmts RCBR", func),
	("func", "FUNC", func_err)
];

fn func(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	let stmts = match &nodes[3] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[4]))
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[4].token().unwrap().loc.end.to_owned();

	Ok(Node::Func(Func { id, stmts, loc }))
}

fn func_err(_nodes: &[Node]) -> Result<Node, Feedback> {
	Err("In 'func', expected '{'".to_owned())
}