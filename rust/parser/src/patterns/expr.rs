use parse::{PatternFunc, Loc, Token};
use bird_utils::*;

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
	pub fn loc(&self) -> &Loc {
		match self {
			Self::AssignExpr(x) => &x.loc,
			Self::BinExpr(x) => &x.loc,
			Self::IfExpr(x) => &x.loc,
			Self::UnaryExpr(x) => &x.loc,

			Self::Literal(x) => &x.loc,
			Self::Id(x) => &x.loc
		}
	}
}

pub static EXPR_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 6] = [
	("expr", "assign_expr", expr),
	("expr", "bin_op", expr),
	("expr", "if_expr", expr),
	("expr", "unary_op", expr),

	("expr", "literal", expr),
	("expr", "ID", expr)
];

fn expr(nodes: &[Node]) -> Result<Node, Feedback> {
	Ok(match &nodes[0] {
		Node::AssignExpr(x) => Node::Expr(Expr::AssignExpr(Box::new(x.to_owned()))),
		Node::BinExpr(x) => Node::Expr(Expr::BinExpr(Box::new(x.to_owned()))),
		Node::IfExpr(x) => Node::Expr(Expr::IfExpr(Box::new(x.to_owned()))),
		Node::UnaryExpr(x) => Node::Expr(Expr::UnaryExpr(Box::new(x.to_owned()))),

		Node::Literal(x) => Node::Expr(Expr::Literal(x.to_owned())),
		Node::Token(x) => Node::Expr(Expr::Id(x.to_owned())),
		
		_ => panic!("If you see this, that means the dev does bad work")
	})
}