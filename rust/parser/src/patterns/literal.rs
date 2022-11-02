use parse::{ASTNode, PatternFunc};

use crate::Node;

#[derive(Debug, Clone)]
pub enum LiteralKind {
	Char,
	Float,
	Int,
	Str
}

#[derive(Debug, Clone)]
pub struct Literal {
	pub kind: LiteralKind,
	pub value: String
}

pub static LITERAL_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("literal", "INT", literal_int),
	("literal", "FLOAT", literal_float),
	("literal", "STR", literal_str),
];

pub fn literal_int(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Int,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

pub fn literal_float(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Float,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

pub fn literal_str(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Str,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}