use parse::PatternFunc;

use crate::Node;

use super::{Literal, IfExpr, AssignExpr, BinExpr, UnaryExpr};

pub static EXPR_PATTERNS: [(&str, &str, PatternFunc<Node>); 6] = [
	("expr", "assign_expr", expr_assign),
	("expr", "bin_op", expr_bin_op),
	("expr", "if_expr", expr_if),
	("expr", "unary_op", expr_unary_op),

	("expr", "literal", expr_literal),
	("expr", "ID", expr_id)
];

#[derive(Debug, Clone)]
pub enum Expr {
	BinExpr(Box<BinExpr>),
	AssignExpr(Box<AssignExpr>),
	IfExpr(Box<IfExpr>),
	UnaryExpr(Box<UnaryExpr>),

	Literal(Literal),
	Id(String)
}

fn expr_assign(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::AssignExpr(x) => Node::Expr(Expr::AssignExpr(Box::new(x.to_owned()))),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_assign'", nodes[0]))
	})
}

fn expr_bin_op(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::BinExpr(x) => Node::Expr(Expr::BinExpr(Box::new(x.to_owned()))),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_bin_op'", nodes[0]))
	})
}

fn expr_if(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::IfExpr(x) => Node::Expr(Expr::IfExpr(Box::new(x.to_owned()))),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_if'", nodes[0]))
	})
}

fn expr_unary_op(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::UnaryExpr(x) => Node::Expr(Expr::UnaryExpr(Box::new(x.to_owned()))),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_assign'", nodes[0]))
	})
}

fn expr_literal(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::Literal(x) => Node::Expr(Expr::Literal(x.to_owned())),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_literal'", nodes[0]))
	})
}

fn expr_id(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::Token(x) => Node::Expr(Expr::Id(x.symbol().to_owned())),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_id'", nodes[0]))
	})
}