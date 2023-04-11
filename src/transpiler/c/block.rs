use crate::parser::exprs::block::Block;

pub fn transpile(input: &Block) -> String {
    let mut exprs = String::new();

    for expr in &input.exprs {
        exprs += &super::transpile(expr);
    }    

    format!("{{{}}}", exprs)
}