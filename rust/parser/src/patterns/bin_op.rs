use parse::PatternFunc;

use crate::Node;

use super::{Expr, Literal, LiteralKind};

pub static BIN_OP_PATTERNS: [(&str, &str, PatternFunc<Node>); 4] = [
	("bin_op", "INT PLUS expr", bin_op_int),
	("bin_op", "INT MINUS expr", bin_op_int),
	("bin_op", "INT MULT expr", bin_op_int),
	("bin_op", "INT DIV expr", bin_op_int)
];

#[derive(Debug, Clone)]
pub struct BinOp {
	pub left: Expr,
	pub op: String,
	pub right: Expr
}

fn bin_op_int(nodes: &[Node]) -> Result<Node, String> {
	let left = match &nodes[0] {
		Node::Token(x) => Expr::Literal(Literal { kind: LiteralKind::Int, value: x.symbol().to_owned() }),
		_ => return Err(format!("Invalid node '{:?}' in 'bin_op_int'", nodes[0]))
	};

	let op = match &nodes[1] {
		Node::Token(x) => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'bin_op_int'", nodes[1]))
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'bin_op_int'", nodes[2]))
	};

	Ok(Node::BinOp(BinOp { left, op, right }))
}