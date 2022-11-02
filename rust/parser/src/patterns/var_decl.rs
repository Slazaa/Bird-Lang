use parse::PatternFunc;

use crate::Node;

use super::Expr;

#[derive(Debug, Clone)]
pub struct VarDecl {
	pub id: String,
	pub expr: Option<Expr>
}

pub static VAR_DECL_PATTERNS: [(&str, &str, PatternFunc<Node>); 2] = [
	("var_decl", "VAR ID SEMI", var_decl),
	("var_decl", "VAR ID EQ expr SEMI", var_decl_expr)
];

pub fn var_decl(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name() == "ID" => token.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	Ok(Node::VarDecl(VarDecl { id, expr: None }))
}

pub fn var_decl_expr(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name() == "ID" => token.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[1]))
	};

	let expr = match &nodes[3] {
		Node::Expr(expr) => Some(expr.to_owned()),
		_ => return Err(format!("Invalid node '{:?}' in 'var'", nodes[3]))
	};

	Ok(Node::VarDecl(VarDecl { id, expr }))
}