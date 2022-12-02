use parse::{PatternFunc, Loc, ASTNode};

use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub struct ConstDecl {
	pub public: bool,
	pub id: String,
	pub val: Expr,
	pub loc: Loc
}

pub static CONST_DECL_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("const_decl", "CONST ID EQ expr SEMI", const_decl),
];

fn const_decl(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let val = match &nodes[3] {
		Node::Expr(expr) => expr.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[4].token().unwrap().loc.end.to_owned();

	Ok(Node::ConstDecl(ConstDecl { public: false, id, val, loc }))
}