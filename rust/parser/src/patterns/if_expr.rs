use parse::{PatternFunc, Loc, ASTNode};

use crate::Node;

use super::{Expr, Stmts};

#[derive(Debug, Clone)]
pub struct IfExpr {
	pub cond: Expr,
	pub stmts: Stmts,
	pub loc: Loc
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

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[4].token().unwrap().loc.end.to_owned();

	Ok(Node::IfExpr(IfExpr { cond, stmts, loc }))
}