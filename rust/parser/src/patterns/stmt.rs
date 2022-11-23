use parse::{PatternFunc, Location};

use crate::Node;

use super::{Expr, Item};

#[derive(Debug, Clone)]
pub enum Stmt {
	Expr(Expr),
	Item(Item)
}

impl Stmt {
	pub fn location(&self) -> Location {
		match self {
			Self::Expr(x) => x.location(),
			Self::Item(x) => x.location()
		}
	}
}

pub static STMT_PATTERNS: [(&str, &str, PatternFunc<Node>); 2] = [
	("stmt", "item", stmt),
	("stmt", "expr", stmt)
];

fn stmt(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Stmt(match nodes[0].to_owned() {
		Node::Expr(x) => Stmt::Expr(x),
		Node::Item(x) => Stmt::Item(x),
		_ => return Err(format!("Invalid node '{:?}' in 'stmt'", nodes[0]))
	}))
}