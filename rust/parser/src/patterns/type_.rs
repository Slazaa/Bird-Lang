use parse::{PatternFunc, Loc, ASTNode};

use bird_utils::*;

use crate::Node;

#[derive(Debug, Clone)]
pub enum PtrKind {
	None,
	Const,
	Mut
}

#[derive(Debug, Clone)]
pub struct Type {
	pub id: String,
	pub ptr_kind: PtrKind,
	pub loc: Loc
}

pub static TYPE_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 3] = [
	("type", "ID", type_),
	("type", "MULT ID", type_ptr),
	("type", "MULT MUT ID", type_ptr_mut)
];

pub fn type_(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[0] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[0].token().unwrap().loc.end.to_owned();

	Ok(Node::Type(Type { id, ptr_kind: PtrKind::None, loc }))
}

pub fn type_ptr(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[1].token().unwrap().loc.end.to_owned();

	Ok(Node::Type(Type { id, ptr_kind: PtrKind::Const, loc }))
}

pub fn type_ptr_mut(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[2] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[2].token().unwrap().loc.end.to_owned();

	Ok(Node::Type(Type { id, ptr_kind: PtrKind::Mut, loc }))
}