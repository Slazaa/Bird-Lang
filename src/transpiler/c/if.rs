use crate::{parser::exprs::r#if::{If, IfBranch}, transpiler::c::block};

pub fn transpile_branch(input: &IfBranch) -> String {
    "else".to_owned() + &match &input {
        IfBranch::ElseIf(expr) => " ".to_owned() + &transpile(expr),
        IfBranch::Else(expr) => block::transpile(expr)
    }
}

pub fn transpile(input: &If) -> String {
    let cond = super::transpile(&input.cond);
    let body = block::transpile(&input.body);
    let branch = match &input.branch {
        Some(branch) => transpile_branch(branch),
        None => "".to_owned()
    };

    format!("if({cond}){body}{branch}")
}