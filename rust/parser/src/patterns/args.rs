use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub struct Args {
	pub args: Vec<Expr>,
	pub loc: Loc
}

pub static ARGS_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 2] = [
	("args", "expr COM args", args),
	("args", "expr", args)
];

fn args(nodes: &[Node]) -> Result<Node, Feedback> {
	if nodes.is_empty() {
		return Ok(Node::Args(Args { args: vec![], loc: Loc::default() }));
	}

	let expr = match &nodes[0] {
		Node::Expr(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let args = match nodes.get(2) {
		Some(Node::Args(x)) => x.args.clone(),
		_ => vec![]
	};

	let mut args_vec = vec![expr.to_owned()];
	args_vec.extend(args.to_owned());

	let mut loc = nodes[0].loc().to_owned();
	loc.end = if args.is_empty() {
		expr.loc().end.to_owned()
	} else {
		args.last().unwrap().loc().end.to_owned()
	};

	Ok(Node::Args(Args { args: args_vec, loc }))
}