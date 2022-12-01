use parse::{PatternFunc, Loc};
use bird_utils::*;

use crate::Node;

use super::{Expr, Item};

#[derive(Debug, Clone)]
pub enum Stmt {
	Expr(Expr),
	Item(Item)
}

impl Stmt {
	pub fn loc(&self) -> &Loc {
		match self {
			Self::Expr(x) => x.loc(),
			Self::Item(x) => x.loc()
		}
	}
}

pub static STMT_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 2] = [
	("stmt", "item", stmt),
	("stmt", "expr SEMI", stmt)
];

fn stmt(nodes: &[Node]) -> Result<Node, Feedback> {
	Ok(Node::Stmt(match nodes[0].to_owned() {
		Node::Expr(x) => Stmt::Expr(x),
		Node::Item(x) => Stmt::Item(x),
		_ => panic!("If you see this, that means the dev does bad work")
	}))
}