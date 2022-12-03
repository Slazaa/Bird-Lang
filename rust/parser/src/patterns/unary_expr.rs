use parse::*;
use bird_utils::*;

use crate::Node;

use super::Expr;

#[derive(Debug, Clone)]
pub struct UnaryExpr {
	pub op: String,
	pub val: Expr,
	pub loc: Loc
}

pub static UNARY_OP_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 3] = [
	("unary_op", "MINUS expr", unary_op),
	("unary_op", "AMP expr", unary_op),
	("unary_op", "MULT expr", unary_op)
];

fn unary_op(nodes: &[Node]) -> Result<Node, Feedback> {
	let op = match &nodes[0] {
		Node::Token(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let val = match &nodes[1] {
		Node::Expr(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[1].loc().end.to_owned();

	Ok(Node::UnaryExpr(UnaryExpr { op: op.symbol, val: val.to_owned(), loc }))
}