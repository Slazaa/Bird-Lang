use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub struct VarDecl {
	pub public: Option<bool>,
	pub id: String,
	pub val: Option<Expr>,
	pub loc: Loc
}

pub static VAR_DECL_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 2] = [
	("var_decl", "VAR ID SEMI", var_decl),
	("var_decl", "VAR ID EQ expr SEMI", var_decl_expr),
];

fn var_decl(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[2].loc().end.to_owned();

	Ok(Node::VarDecl(VarDecl { public: None, id, val: None, loc }))
}

fn var_decl_expr(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let val = match &nodes[3] {
		Node::Expr(expr) => Some(expr.to_owned()),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[4].loc().end.to_owned();

	Ok(Node::VarDecl(VarDecl { public: None, id, val, loc }))
}