use parse::*;
use bird_utils::*;

use crate::Node;

use super::Expr;

#[derive(Debug, Clone)]
pub struct AssignExpr {
	pub left: Expr,
	pub right: Expr,
	pub loc: Loc
}

pub static ASSIGN_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("assign_expr", "ID EQ expr SEMI", assign_id)
];

fn assign_id(nodes: &[Node]) -> Result<Node, Feedback> {
	let left = match &nodes[0] {
		Node::Token(x) => x,
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[3].loc().end.to_owned();

	Ok(Node::AssignExpr(AssignExpr { left: Expr::Id(left.to_owned()), right: right.to_owned(), loc }))
}