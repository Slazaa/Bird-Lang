use parse::PatternFunc;

use crate::Node;

use super::{Literal, IfExpr, AssignExpr, BinOp};

pub static EXPR_PATTERNS: [(&str, &str, PatternFunc<Node>); 5] = [
	("expr", "assign_expr", expr_assign),
	("expr", "bin_op", expr_bin_op),
	("expr", "if_expr", expr_if),

	("expr", "literal", expr_literal),
	("expr", "ID", expr_id)
];

#[derive(Debug, Clone)]
pub enum Expr {
	BinOp(Box<BinOp>),
	AssignExpr(Box<AssignExpr>),
	Id(String),
	IfExpr(Box<IfExpr>),
	Literal(Literal)
}

fn expr_assign(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::AssignExpr(x) => Node::Expr(Expr::AssignExpr(Box::new(x.to_owned()))),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_assign'", nodes[0]))
	})
}

fn expr_bin_op(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::BinOp(x) => Node::Expr(Expr::BinOp(Box::new(x.to_owned()))),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_bin_op'", nodes[0]))
	})
}

fn expr_if(nodes: &[Node]) -> Result<Node, String> {
	Ok(match &nodes[0] {
		Node::IfExpr(x) => Node::Expr(Expr::IfExpr(Box::new(x.to_owned()))),
		_ => return Err(format!("Invalid node '{:?}' in 'expr_if'", nodes[0]))
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