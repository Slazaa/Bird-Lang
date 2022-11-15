use parse::PatternFunc;

use crate::Node;

use super::Stmts;

#[derive(Debug, Clone)]
pub struct Func {
	pub id: String,
	pub stmts: Stmts
}

pub static FUNC_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("func", "FUNC ID LPAR RPAR LCBR stmts RCBR", func)
];

fn func(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name() == "ID" => token.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	let stmts = match &nodes[5] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[4]))
	};

	Ok(Node::Func(Func { id, stmts }))
}