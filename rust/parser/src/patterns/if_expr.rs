use parse::PatternFunc;

use crate::Node;

use super::{Expr, Stmts};

#[derive(Debug, Clone)]
pub struct IfExpr {
	pub cond: Expr,
	pub stmts: Stmts
}

pub static IF_EXPR_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("if_expr", "IF expr LCBR stmts RCBR", if_expr)
];

fn if_expr(nodes: &[Node]) -> Result<Node, String> {
	let cond = match &nodes[1] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'if_expr'", nodes[1]))
	};

	let stmts = match &nodes[3] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'if_expr'", nodes[3]))
	};

	Ok(Node::IfExpr(IfExpr { cond, stmts }))
}