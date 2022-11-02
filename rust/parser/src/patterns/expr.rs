use parse::PatternFunc;

use crate::Node;

use super::Literal;

pub static EXPR_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("expr", "literal", expr_literal)
];

#[derive(Debug, Clone)]
pub enum Expr {
	Literal(Literal),
	BinExpr()
}

pub fn expr_literal(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::Literal(x) => Node::Expr(Expr::Literal(x.to_owned())),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[0]))
	})
}