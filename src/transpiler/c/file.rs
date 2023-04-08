use crate::parser::exprs::{Expr, file::File};

use super::fn_decl;

pub fn transpile(input: &File) -> String {
    let mut res = String::new();

    for expr in &input.exprs {
        res += &match expr {
            Expr::BoxDecl(x) => if let Ok(expr) = fn_decl::transpile(x) { expr } else { super::transpile(expr) }
            _ => super::transpile(expr)
        };
    }

    res
}
