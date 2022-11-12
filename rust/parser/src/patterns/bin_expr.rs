use parse::PatternFunc;

use crate::Node;

use super::{Expr, Literal, LiteralKind};

#[derive(Debug, Clone)]
pub struct BinExpr {
	pub left: Expr,
	pub op: String,
	pub right: Expr
}

pub static BIN_OP_PATTERNS: [(&str, &str, PatternFunc<Node>); 4] = [
	("bin_op", "INT PLUS expr", bin_op_int),
	("bin_op", "INT MINUS expr", bin_op_int),
	("bin_op", "INT MULT expr", bin_op_int),
	("bin_op", "INT DIV expr", bin_op_int)
];

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

	Ok(Node::BinExpr(BinExpr { left, op, right }))
}