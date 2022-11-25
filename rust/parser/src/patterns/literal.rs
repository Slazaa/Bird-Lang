use parse::{PatternFunc, Loc};

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

pub static LITERAL_PATTERNS: [(&str, &str, PatternFunc<Node>); 5] = [
	("literal", "BOOL", literal),
	("literal", "INT", literal),
	("literal", "FLT", literal),
	("literal", "CHR", literal),
	("literal", "STR", literal)
];

fn literal(nodes: &[Node]) -> Result<Node, String> {
	let token = match &nodes[0] {
		Node::Token(x) => x,
		_ => return Err(format!("In 'literal', expected 'BOOL', 'INT', 'FLT', 'CHR' or 'STR', found '{:?}'", nodes[0]))
	};

	let kind = match token.name.as_str() {
		"BOOL" => LiteralKind::Bool,
		"INT" => LiteralKind::Int,
		"FLT" => LiteralKind::Flt,
		"CHR" => LiteralKind::Chr,
		"STR" => LiteralKind::Str,
		
		_ => return Err(format!("In 'literal', expected 'BOOL', 'INT', 'FLT', 'CHR' or 'STR', found '{:?}'", token))
	};

	Ok(Node::Literal(Literal { kind, value: token.symbol.to_owned(), loc: token.loc.to_owned() }))
}