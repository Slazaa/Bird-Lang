use parse::PatternFunc;

use crate::Node;

use super::Expr;

pub static UNARY_OP_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("unary_op", "MINUS expr", unary_op_minus),
	("unary_op", "AMP ID", unary_op_ref),
	("unary_op", "MULT ID", unary_op_deref)
];

#[derive(Debug, Clone)]
pub struct UnaryExpr {
	pub op: String,
	pub val: Expr
}

fn unary_op_minus(nodes: &[Node]) -> Result<Node, String> {
	let op = match &nodes[0] {
		Node::Token(x) => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'unary_op_minus'", nodes[0]))
	};

	let val = match &nodes[1] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'unary_op_minus'", nodes[1]))
	};

	Ok(Node::UnaryExpr(UnaryExpr { op, val }))
}

fn unary_op_ref(nodes: &[Node]) -> Result<Node, String> {
	let op = match &nodes[0] {
		Node::Token(x) => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'unary_op_deref'", nodes[0]))
	};

	let val = match &nodes[1] {
		Node::Token(x) => Expr::Id(x.symbol().to_owned()),
		_ => return Err(format!("Invalid node '{:?}' in 'unary_op_deref'", nodes[1]))
	};

	Ok(Node::UnaryExpr(UnaryExpr { op, val }))
}

fn unary_op_deref(nodes: &[Node]) -> Result<Node, String> {
	let op = match &nodes[0] {
		Node::Token(x) => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'unary_op_deref'", nodes[0]))
	};

	let val = match &nodes[1] {
		Node::Token(x) => Expr::Id(x.symbol().to_owned()),
		_ => return Err(format!("Invalid node '{:?}' in 'unary_op_deref'", nodes[1]))
	};

	Ok(Node::UnaryExpr(UnaryExpr { op, val }))
}