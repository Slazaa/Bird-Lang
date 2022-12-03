use parse::*;
use bird_utils::*;

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
	pub value: String,
	pub loc: Loc
}

pub static LITERAL_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 5] = [
	("literal", "BOOL", literal),
	("literal", "INT", literal),
	("literal", "FLT", literal),
	("literal", "CHR", literal),
	("literal", "STR", literal)
];

fn literal(nodes: &[Node]) -> Result<Node, Feedback> {
	let token = match &nodes[0] {
		Node::Token(x) => x,
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let kind = match token.name.as_str() {
		"BOOL" => LiteralKind::Bool,
		"INT" => LiteralKind::Int,
		"FLT" => LiteralKind::Flt,
		"CHR" => LiteralKind::Chr,
		"STR" => LiteralKind::Str,
		
		_ => panic!("If you see this, that means the dev does bad work")
	};

	Ok(Node::Literal(Literal { kind, value: token.symbol.to_owned(), loc: token.loc.to_owned() }))
}