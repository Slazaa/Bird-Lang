use parse::{ASTNode, PatternFunc};

use crate::Node;

#[derive(Debug, Clone)]
pub enum LiteralKind {
	Bool,
	Int,
	Flt,
	Chr,
	Str
}

#[derive(Debug, Clone)]
pub struct Literal {
	pub kind: LiteralKind,
	pub value: String
}

pub static LITERAL_PATTERNS: [(&str, &str, PatternFunc<Node>); 5] = [
	("literal", "BOOL", literal_bool),
	("literal", "INT", literal_int),
	("literal", "FLT", literal_flt),
	("literal", "CHR", literal_chr),
	("literal", "STR", literal_str)
];

fn literal_bool(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Bool,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

fn literal_int(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Int,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

fn literal_chr(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Chr,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

fn literal_flt(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Flt,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}

fn literal_str(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Literal(Literal {
		kind: LiteralKind::Str,
		value: nodes[0].token().unwrap().symbol().to_owned()
	}))
}