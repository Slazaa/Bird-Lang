use parse::{PatternFunc, Location, ASTNode};

use crate::Node;

use super::Stmts;

#[derive(Debug, Clone)]
pub struct Func {
	pub id: String,
	pub stmts: Stmts,
	pub location: Location
}

pub static FUNC_PATTERNS: [(&str, &str, PatternFunc<Node>); 2] = [
	("func", "FUNC ID LCBR stmts RCBR", func),
	("func", "FUNC", func_err)
];

fn func(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	let stmts = match &nodes[3] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[4]))
	};

	Ok(Node::Func(Func { id, stmts, location: Location { start: nodes.first().unwrap().token().unwrap().location.start, end: nodes.last().unwrap().token().unwrap().location.end } }))
}

fn func_err(_nodes: &[Node]) -> Result<Node, String> {
	Err("In 'func', expected '{'".to_owned())
}