use crate::parser::exprs::ident::Ident;

pub fn transpile(input: &Ident) -> String {
	input.value.to_owned()
}
