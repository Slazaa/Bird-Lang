use parse::{PatternFunc, Location, ASTNode};

use crate::Node;

use super::Expr;

#[derive(Debug, Clone)]
pub struct VarDecl {
	pub id: String,
	pub var_type: Option<String>,
	pub val: Option<Expr>,
	pub location: Location
}

pub static VAR_DECL_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	//("var_decl", "VAR ID SEMI", var_decl),
	("var_decl", "VAR ID EQ expr SEMI", var_decl_expr),
	("var_decl", "VAR ID COL ID SEMI", var_decl_typed),
	("var_decl", "VAR ID COL ID EQ expr SEMI", var_decl_typed_expr)
];

fn _var_decl(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	Ok(Node::VarDecl(VarDecl { id, var_type: None, val: None, location: Location { start: nodes.first().unwrap().token().unwrap().location.start, end: nodes.last().unwrap().token().unwrap().location.end } }))
}

fn var_decl_expr(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	let val = match &nodes[3] {
		Node::Expr(expr) => Some(expr.to_owned()),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[3]))
	};

	Ok(Node::VarDecl(VarDecl { id, var_type: None, val, location: Location { start: nodes.first().unwrap().token().unwrap().location.start, end: nodes.last().unwrap().token().unwrap().location.end } }))
}

fn var_decl_typed(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	let var_type = match &nodes[3] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	Ok(Node::VarDecl(VarDecl { id, var_type: Some(var_type), val: None, location: Location { start: nodes.first().unwrap().token().unwrap().location.start, end: nodes.last().unwrap().token().unwrap().location.end } }))
}

fn var_decl_typed_expr(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	let var_type = match &nodes[3] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	let val = match &nodes[5] {
		Node::Expr(expr) => Some(expr.to_owned()),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[3]))
	};

	Ok(Node::VarDecl(VarDecl { id, var_type: Some(var_type), val, location: Location { start: nodes.first().unwrap().token().unwrap().location.start, end: nodes.last().unwrap().token().unwrap().location.end } }))
}