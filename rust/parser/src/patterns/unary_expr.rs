use parse::{PatternFunc, Location};

use crate::Node;

use super::Expr;

#[derive(Debug, Clone)]
pub struct UnaryExpr {
	pub op: String,
	pub val: Expr,
	pub location: Location
}

pub static UNARY_OP_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("unary_op", "MINUS expr", unary_op),
	("unary_op", "AMP expr", unary_op),
	("unary_op", "MULT expr", unary_op)
];

fn unary_op(nodes: &[Node]) -> Result<Node, String> {
	let op = match &nodes[0] {
		Node::Token(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'unary_op'", nodes[0]))
	};

	let val = match &nodes[1] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'unary_op'", nodes[1]))
	};

	Ok(Node::UnaryExpr(UnaryExpr { op: op.symbol, val: val.to_owned(), location: Location { start: op.location.start, end: val.location().end } }))
}