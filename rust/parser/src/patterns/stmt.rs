use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub enum Stmt {
	Import(Import),

	Expr(Expr),
	Item(Item)
}

impl Stmt {
	pub fn loc(&self) -> &Loc {
		match self {
			Self::Import(x) => &x.loc,

			Self::Expr(x) => x.loc(),
			Self::Item(x) => x.loc(),
		}
	}
}

pub static STMT_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 7] = [
	("stmt", "if_expr", stmt),

	("stmt", "expr SEMI", stmt),
	("stmt", "item", stmt),

	("program_stmt", "import", import_stmt),
	("program_stmt", "PUB item", pub_item),
	("program_stmt", "item", priv_item),
	("program_stmt", "stmt", program_stmt)
];

fn stmt(nodes: &[Node]) -> Result<Node, Feedback> {
	Ok(Node::Stmt(match &nodes[0] {
		Node::IfExpr(x) => Stmt::Expr(Expr::IfExpr(Box::new(x.to_owned()))),

		Node::Expr(x) => Stmt::Expr(x.to_owned()),
		Node::Item(x) => Stmt::Item(x.to_owned()),
		_ => panic!("If you see this, that means the dev does bad work")
	}))
}

fn import_stmt(nodes: &[Node]) -> Result<Node, Feedback> {
	Ok(Node::Stmt(match &nodes[0] {
		Node::Import(x) => Stmt::Import(x.to_owned()),
		_ => panic!("If you see this, that means the dev does bad work")
	}))
}

fn pub_item(nodes: &[Node]) -> Result<Node, Feedback> {
	Ok(Node::Stmt(Stmt::Item(match &nodes[1] {
		Node::Item(x) => {
			let mut x = x.to_owned();
			*x.public_mut() = Some(true);
			x
		}
		_ => panic!("If you see this, that means the dev does bad work")
	})))
}

fn priv_item(nodes: &[Node]) -> Result<Node, Feedback> {
	Ok(Node::Stmt(Stmt::Item(match &nodes[0] {
		Node::Item(x) => {
			let mut x = x.to_owned();
			*x.public_mut() = Some(false);
			x
		}
		_ => panic!("If you see this, that means the dev does bad work")
	})))
}

fn program_stmt(nodes: &[Node]) -> Result<Node, Feedback> {
	Ok(nodes[0].to_owned())
}