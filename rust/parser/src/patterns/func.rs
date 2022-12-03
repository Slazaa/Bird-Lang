use parse::{PatternFunc, Loc, ASTNode};
use bird_utils::*;

use crate::Node;

use super::Stmts;

#[derive(Debug, Clone)]
pub struct Func {
	pub public: Option<bool>,
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
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let stmts = match &nodes[3] {
		Node::Stmts(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[4].token().unwrap().loc.end.to_owned();

	Ok(Node::Func(Func { public: None, id, stmts, loc }))
}

fn func_err(nodes: &[Node]) -> Result<Node, Feedback> {
	let (loc, found) = match nodes.get(1) {
		Some(x) => (x.loc(), Some(x.token().unwrap().symbol.as_str())),
		None => (nodes[0].loc(), None)
	};

	Err(Error::expected(loc, "'{'", found))
}