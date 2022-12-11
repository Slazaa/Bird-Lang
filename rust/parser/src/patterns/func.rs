use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub struct Func {
	pub public: Option<bool>,
	pub id: String,
	pub params: Option<Fields>,
	pub ret_type: Option<Type>,
	pub stmts: Stmts,
	pub loc: Loc
}

pub static FUNC_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 4] = [
	("func", "FUNC ID LCBR stmts RCBR", func),
	("func", "FUNC ID SARR type LCBR stmts RCBR", func_ret_type),
	("func", "FUNC ID LPAR fields RPAR LCBR stmts RCBR", func_params),
	("func", "FUNC ID LPAR fields RPAR SARR type LCBR stmts RCBR", func_ret_type_params),
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

	Ok(Node::Func(Func { public: None, params: None, ret_type: None, id, stmts, loc }))
}

fn func_ret_type(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let ret_type = Some(match &nodes[3] {
		Node::Type(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	});

	let stmts = match &nodes[5] {
		Node::Stmts(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[6].loc().end.to_owned();

	Ok(Node::Func(Func { public: None, params: None, ret_type, id, stmts, loc }))
}

fn func_ret_type_params(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let params = Some(match &nodes[3] {
		Node::Fields(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	});

	let ret_type = Some(match &nodes[6] {
		Node::Type(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	});

	let stmts = match &nodes[8] {
		Node::Stmts(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[9].loc().end.to_owned();

	Ok(Node::Func(Func { public: None, params, ret_type, id, stmts, loc }))
}

fn func_params(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let params = Some(match &nodes[3] {
		Node::Fields(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	});

	let stmts = match &nodes[6] {
		Node::Stmts(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[7].loc().end.to_owned();

	Ok(Node::Func(Func { public: None, params, ret_type: None, id, stmts, loc }))
}