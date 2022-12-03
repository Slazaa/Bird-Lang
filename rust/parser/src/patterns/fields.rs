use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub struct Fields {
	pub fields: Vec<Field>,
	pub loc: Loc
}

pub static FIELDS_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 2] = [
	("fields", "field COM fields", fields),
	("fields", "field", fields)
];

fn fields(nodes: &[Node]) -> Result<Node, Feedback> {
	if nodes.is_empty() {
		return Ok(Node::Fields(Fields { fields: vec![], loc: Loc::default() }));
	}

	let field = match &nodes[0] {
		Node::Field(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let fields = match nodes.get(2) {
		Some(Node::Fields(x)) => x.fields.clone(),
		_ => vec![]
	};

	let mut fields_vec = vec![field.to_owned()];
	fields_vec.extend(fields.to_owned());

	let mut loc = field.loc.to_owned();
	loc.end = if fields.is_empty() {
		field.loc.end.to_owned()
	} else {
		fields.last().unwrap().loc.end.to_owned()
	};

	Ok(Node::Fields(Fields { fields: fields_vec, loc }))
}