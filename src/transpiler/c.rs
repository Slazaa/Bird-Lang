use crate::parser::exprs::{Expr, file::File};

pub mod file;
pub mod fn_decl;

pub fn transpile(input: &Expr) -> String {
    todo!()
}

pub fn transpile_file(input: &File) -> String {
    file::transpile(input)
}
