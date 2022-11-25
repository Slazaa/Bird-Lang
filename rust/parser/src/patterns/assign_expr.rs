use parse::{PatternFunc, Loc, ASTNode};

use crate::Node;

use super::Expr;

#[derive(Debug, Clone)]
pub struct AssignExpr {
	pub left: Expr,
	pub right: Expr,
	pub loc: Loc
}

pub static ASSIGN_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("assign_expr", "ID EQ expr SEMI", assign_id)
];

fn assign_id(nodes: &[Node]) -> Result<Node, String> {
	let left = match &nodes[0] {
		Node::Token(x) => x,
		_ => return Err(format!("In 'assign_id', expected 'ID', found '{:?}'", nodes[0]))
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x.to_owned(),
		_ => return Err(format!("In 'assign_id', expected 'expr', founc '{:?}'", nodes[2]))
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[3].token().unwrap().loc.end.to_owned();

	Ok(Node::AssignExpr(AssignExpr { left: Expr::Id(left.to_owned()), right: right.to_owned(), loc }))
}