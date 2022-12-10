use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub struct Func {
	pub public: Option<bool>,
	pub id: String,
	pub params: Option<Fields>,
	pub stmts: Stmts,
	pub loc: Loc
}

pub static FUNC_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("func", "FUNC ID LCBR stmts RCBR", func),
	("func", "FUNC ID LPAR fields RPAR LCBR stmts RCBR", func_params)
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

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[4].loc().end.to_owned();

	Ok(Node::Func(Func { public: None, params: None, id, stmts, loc }))
}

fn func_params(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let params = match &nodes[3] {
		Node::Fields(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let stmts = match &nodes[6] {
		Node::Stmts(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[7].loc().end.to_owned();

	Ok(Node::Func(Func { public: None, params: Some(params), id, stmts, loc }))
}