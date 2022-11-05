use parse::PatternFunc;

use crate::Node;

use super::{Literal, LiteralKind};

pub static EXPR_PATTERNS: [(&str, &str, PatternFunc<Node>); 5] = [
	("expr", "INT PLUS expr", expr_bin_op_int),
	("expr", "INT MINUS expr", expr_bin_op_int),
	("expr", "INT MULT expr", expr_bin_op_int),
	("expr", "INT DIV expr", expr_bin_op_int),
	
	("expr", "literal", expr_literal)
];

#[derive(Debug, Clone)]
pub struct BinOp {
	pub left: Box<Expr>,
	pub op: String,
	pub right: Box<Expr>
}

#[derive(Debug, Clone)]
pub enum Expr {
	Literal(Literal),
	BinOp(BinOp)
}

fn expr_bin_op_int(nodes: &[Node]) -> Result<Node, String> {
	let left = match &nodes[0] {
		Node::Token(x) => Expr::Literal(Literal { kind: LiteralKind::Int, value: x.symbol().to_owned() }),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[0]))
	};

	let op = match &nodes[1] {
		Node::Token(x) => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[2]))
	};

	Ok(Node::Expr(Expr::BinOp(BinOp { left: Box::new(left), op, right: Box::new(right) })))
}

fn expr_literal(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::Literal(x) => Node::Expr(Expr::Literal(x.to_owned())),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[0]))
	})
}