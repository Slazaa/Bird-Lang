use parse::{PatternFunc, Location};

use crate::Node;

use super::Expr;

#[derive(Debug, Clone)]
pub struct AssignExpr {
	pub left: Expr,
	pub right: Expr,
	pub location: Location
}

pub static ASSIGN_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("assign_expr", "ID EQ expr SEMI", assign_id)
];

fn assign_id(nodes: &[Node]) -> Result<Node, String> {
	let left = match &nodes[0] {
		Node::Token(x) => x,
		_ => return Err(format!("Invalid node '{:?}' in 'assign_id'", nodes[0]))
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'assign_id'", nodes[2]))
	};

	Ok(Node::AssignExpr(AssignExpr { left: Expr::Id(left.to_owned()), right: right.to_owned(), location: Location { start: left.location.start, end: right.location().end } }))
}