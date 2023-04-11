use crate::parser::exprs::r#return::Return;

pub fn transpile(input: &Return) -> String {
    format!("return {};", super::transpile(&input.value))
}