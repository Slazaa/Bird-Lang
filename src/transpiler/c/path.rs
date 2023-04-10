use crate::parser::exprs::path::Path;

pub fn transpile(input: &Path) -> String {
	let mut res = if input.exprs.is_empty() {
		String::new()
	} else {
		super::transpile(&input.exprs[0])
	};

	for expr in input.exprs.iter().skip(1) {
		res += &format!(".{}", super::transpile(expr));
	}

	res
}
