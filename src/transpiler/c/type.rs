use crate::parser::exprs::{Expr, r#type::Type};

use super::path;

pub fn transpile(input: &Type) -> String {
    match &input.value {
        Expr::Path(value) => path::transpile(value),
        _ => todo!()
    }
}
