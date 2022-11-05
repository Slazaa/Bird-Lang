use parse::PatternFunc;

use crate::Node;

use super::{Literal, LiteralKind, IfExpr};

pub static EXPR_PATTERNS: [(&str, &str, PatternFunc<Node>); 6] = [
	("expr", "INT PLUS expr", expr_bin_op_int),
	("expr", "INT MINUS expr", expr_bin_op_int),
	("expr", "INT MULT expr", expr_bin_op_int),
	("expr", "INT DIV expr", expr_bin_op_int),

	("expr", "literal", expr_literal),

	("expr", "if_expr", expr_if)
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
	BinOp(BinOp),
	IfExpr(Box<IfExpr>)
}

fn expr_bin_op_int(nodes: &[Node]) -> Result<Node, String> {
	let left = match &nodes[0] {
		Node::Token(x) => Expr::Literal(Literal { kind: LiteralKind::Int, value: x.symbol().to_owned() }),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_bin_op_int'", nodes[0]))
	};

	let op = match &nodes[1] {
		Node::Token(x) => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_bin_op_int'", nodes[1]))
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_bin_op_int'", nodes[2]))
	};

	Ok(Node::Expr(Expr::BinOp(BinOp { left: Box::new(left), op, right: Box::new(right) })))
}

fn expr_literal(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::Literal(x) => Node::Expr(Expr::Literal(x.to_owned())),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_literal'", nodes[0]))
	})
}

fn expr_if(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::IfExpr(x) => Node::Expr(Expr::IfExpr(Box::new(x.to_owned()))),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_if'", nodes[0]))
	})
}