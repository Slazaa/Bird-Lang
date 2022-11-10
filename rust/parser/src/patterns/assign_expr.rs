use parse::PatternFunc;

use crate::Node;

use super::Expr;

pub static ASSIGN_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("assign_expr", "ID EQ expr SEMI", assign_id)
];

#[derive(Debug, Clone)]
pub struct AssignExpr {
	pub left: Expr,
	pub right: Expr
}

fn assign_id(nodes: &[Node]) -> Result<Node, String> {
	let left = match &nodes[0] {
		Node::Token(x) => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'assign'", nodes[0]))
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'assign'", nodes[2]))
	};

	Ok(Node::AssignExpr(AssignExpr { left: Expr::Id(left), right }))
}