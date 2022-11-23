use parse::{PatternFunc, Location, Token};

use crate::Node;

use super::{Literal, IfExpr, AssignExpr, BinExpr, UnaryExpr};

#[derive(Debug, Clone)]
pub enum Expr {
	AssignExpr(Box<AssignExpr>),
	BinExpr(Box<BinExpr>),
	IfExpr(Box<IfExpr>),
	UnaryExpr(Box<UnaryExpr>),

	Literal(Literal),
	Id(Token)
}

impl Expr {
	pub fn location(&self) -> Location {
		match self {
			Self::AssignExpr(x) => x.location,
			Self::BinExpr(x) => x.location,
			Self::IfExpr(x) => x.location,
			Self::UnaryExpr(x) => x.location,

			Self::Literal(x) => x.location,
			Self::Id(x) => x.location
		}
	}
}

pub static EXPR_PATTERNS: [(&str, &str, PatternFunc<Node>); 6] = [
	("expr", "assign_expr", expr),
	("expr", "bin_op", expr),
	("expr", "if_expr", expr),
	("expr", "unary_op", expr),

	("expr", "literal", expr),
	("expr", "ID", expr)
];

fn expr(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::AssignExpr(x) => Node::Expr(Expr::AssignExpr(Box::new(x.to_owned()))),
		Node::BinExpr(x) => Node::Expr(Expr::BinExpr(Box::new(x.to_owned()))),
		Node::IfExpr(x) => Node::Expr(Expr::IfExpr(Box::new(x.to_owned()))),
		Node::UnaryExpr(x) => Node::Expr(Expr::UnaryExpr(Box::new(x.to_owned()))),

		Node::Literal(x) => Node::Expr(Expr::Literal(x.to_owned())),
		Node::Token(x) => Node::Expr(Expr::Id(x.to_owned())),
		_ => return Err(format!("Invalid node '{:?}' in 'expr'", nodes[0]))
	})
}