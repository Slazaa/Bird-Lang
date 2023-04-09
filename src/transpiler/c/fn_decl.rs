use crate::parser::exprs::{Expr, fn_decl::ParamDecl, box_decl::BoxDecl, vis::Vis};
use super::{ident, r#type};

pub fn transpile_param_decl(input: &ParamDecl) -> String {
    let ident = ident::transpile(&input.ident);
    let r#type = r#type::transpile(&input.r#type);

    format!("{type} {ident}")
}

pub fn transpile_param_decls(input: &[ParamDecl]) -> String {
    let mut res = if input.is_empty() {
        String::new()
    } else {
        transpile_param_decl(&input[0])
    };

    for input in input.iter().skip(1) {
        res += &format!(", {}", transpile_param_decl(input));
    }

    res
}

pub fn transpile_sig(input: &BoxDecl) -> Result<String, ()> {
    let ident = ident::transpile(&input.ident);

    let value = match &input.value {
        Some(Expr::FnDecl(x)) => x,
        _ => return Err(())
    };

    let inputs = transpile_param_decls(&value.inputs);

    let output = match &value.output {
        Some(output) => r#type::transpile(output),
        None => "void".to_owned()
    };

    Ok(format!("{output} {ident}({inputs});"))
}

pub fn transpile(input: &BoxDecl) -> Result<String, ()> {
    let vis = match input.vis {
        Vis::Private => "static ".to_string(),
        Vis::Public => "".to_string()
    };

    let ident = ident::transpile(&input.ident);

    let value = match &input.value {
        Some(Expr::FnDecl(x)) => x,
        _ => return Err(())
    };

    let inputs = transpile_param_decls(&value.inputs);

    let output = match &value.output {
        Some(output) => r#type::transpile(output),
        None => "void".to_owned()
    };

    Ok(format!("{vis}{output} {ident}({inputs}) {{}}"))
}
