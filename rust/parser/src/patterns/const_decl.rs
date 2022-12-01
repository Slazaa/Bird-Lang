use parse::{PatternFunc, Loc, ASTNode};

use bird_utils::*;

use crate::Node;

use super::{Expr, Type};

#[derive(Debug, Clone)]
pub struct ConstDecl {
	pub id: String,
	pub var_type: Option<Type>,
	pub val: Expr,
	pub loc: Loc
}

pub static CONST_DECL_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 2] = [
	("const_decl", "CONST ID EQ expr SEMI", const_decl),
	("const_decl", "CONST ID COL type EQ expr SEMI", const_decl_typed_expr)
];

fn const_decl(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let val = match &nodes[3] {
		Node::Expr(expr) => Some(expr.to_owned()),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[4].token().unwrap().loc.end.to_owned();

	Ok(Node::ConstDecl(ConstDecl { id, var_type: None, val, loc }))
}

fn const_decl_typed_expr(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let var_type = match &nodes[3] {
		Node::Type(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let val = match &nodes[5] {
		Node::Expr(expr) => Some(expr.to_owned()),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[6].token().unwrap().loc.end.to_owned();

	Ok(Node::ConstDecl(ConstDecl { id, var_type: Some(var_type), val, loc }))
}