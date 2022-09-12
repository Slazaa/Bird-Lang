pub mod utils;

use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write as _;
use std::path::Path;

use crate::bird::constants::compile::FUNC_PREFIX;
use crate::bird::feedback::*;
use crate::bird::parser::*;
use crate::bird::SRC_FOLDER;

pub static OUTPUT_FOLDER: &str = "c";

pub struct Compiler {
    main_file: File,
    func_protos: Vec<Node>,
}

impl Compiler {
    pub fn compile(ast: &Node, file_path: &Path) -> Result<(), Feedback> {
        let mut output = file_path.to_path_buf();
        let parent_folder = output
            .parent()
            .ok_or_else(|| Error::invalid_syntax(None, "Invalid path"))?;

        if parent_folder.to_str().unwrap() == SRC_FOLDER {
            let filename = file_path.file_name().unwrap().to_str().unwrap();
            output = Path::new(filename).to_path_buf();
        }

        output = Path::new(OUTPUT_FOLDER).join(output);
        output.set_extension("c");

        {
            let parent_folder = output
                .parent()
                .ok_or_else(|| Error::invalid_syntax(None, "Invalid path"))?;

            if !Path::new(parent_folder).exists() && fs::create_dir_all(parent_folder).is_err() {
                return Err(Error::unspecified(&format!(
                    "Failed creating '{}' directory",
                    parent_folder.display()
                )));
            }
        }

        let main_file = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(output)
        {
            Ok(x) => x,
            Err(_) => return Err(Error::unspecified("Failed creating 'main.c' file")),
        };

        let mut compiler = Self {
            main_file,
            func_protos: Vec::new(),
        };

        if write!(compiler.main_file, "{}", self::utils::utils()).is_err() {
            return Err(Error::unspecified("Failed writing to 'main.c' file"));
        }

        let res = compiler.eval(ast)?;
        let func_protos = compiler.func_protos.clone();

        for proto in func_protos {
            let proto = compiler.eval(&proto)?;

            if write!(compiler.main_file, "{}", proto).is_err() {
                return Err(Error::unspecified("Failed writing to 'main.c' file"));
            }
        }

        if write!(
            compiler.main_file,
            "{}int main(int argc, char** argv){{{}main();return 0;}}",
            res, FUNC_PREFIX
        )
        .is_err()
        {
            return Err(Error::unspecified("Failed writing to 'main.c' file"));
        }

        Ok(())
    }

    fn eval(&mut self, node: &Node) -> Result<String, Feedback> {
        match node {
            Node::Undefined => Err(Error::unspecified("Found undefined node")),

            Node::Program(body) => self.program(body),

            Node::Literal(value, ..) => self.literal(value),
            Node::Identifier(value, ..) => self.indentifier(value),
            Node::Operator(value, ..) => self.operator(value),
            Node::Block(nodes) => self.block(nodes),

            Node::UnaryExpr {
                operator,
                node,
                wrapped,
            } => self.unary_expr(operator, node, *wrapped),
            Node::BinExpr {
                operator,
                left,
                right,
                wrapped,
            } => self.bin_expr(operator, left, right, *wrapped),

            Node::Field {
                identifier,
                filed_type,
            } => self.field(identifier, filed_type),

            Node::FuncProto {
                identifier,
                params,
                return_type,
                ..
            } => self.func_proto(identifier, params, return_type),

            Node::FuncItem { proto, body } => self.func_item(proto, body),
            Node::VarItem {
                identifier,
                var_type,
                value,
                ..
            } => self.var_item(identifier, var_type, value),
            Node::TypeItem {
                identifier, value, ..
            } => self.type_item(identifier, value),
            Node::StructItem {
                identifier, fields, ..
            } => self.struct_item(identifier, fields),

            Node::Assignment {
                destination,
                operator,
                value,
            } => self.assignment(destination, operator, &*value),

            Node::FuncCall { identifier, params } => self.func_call(identifier, params),

            Node::IfStmt { condition, body } => self.if_statement(condition, body),
            Node::LoopStmt { condition, body } => self.loop_statement(condition, body),
            Node::ReturnStmt { expr } => self.return_stmt(expr),

            Node::Type { identifier } => self.type_node(identifier),
            Node::TypePtr {
                identifier,
                mutable,
            } => self.type_ptr_node(identifier, *mutable),
        }
    }

    fn program(&mut self, body: &Node) -> Result<String, Feedback> {
        match body {
            Node::Block(nodes) => self.block(nodes),
            _ => Err(Error::unspecified("Invalid node")),
        }
    }

    fn literal(&mut self, value: &str) -> Result<String, Feedback> {
        Ok(value.to_owned())
    }

    fn indentifier(&mut self, value: &str) -> Result<String, Feedback> {
        let value = match value {
            "main" => format!("{}{}", FUNC_PREFIX, value),
            _ => value.to_owned(),
        };

        Ok(value)
    }

    fn operator(&mut self, value: &str) -> Result<String, Feedback> {
        Ok(value.to_owned())
    }

    fn block(&mut self, nodes: &Vec<Node>) -> Result<String, Feedback> {
        let mut res = String::new();

        for node in nodes {
            res.push_str(&self.eval(node)?);

            match node {
                Node::Literal(..)
                | Node::Identifier(..)
                | Node::Operator(..)
                | Node::UnaryExpr { .. }
                | Node::BinExpr { .. }
                | Node::VarItem { .. }
                | Node::Assignment { .. }
                | Node::FuncCall { .. }
                | Node::ReturnStmt { .. } => res.push(';'),
                _ => (),
            }
        }

        Ok(res)
    }

    fn unary_expr(
        &mut self,
        operator: &Node,
        node: &Node,
        wrapped: bool,
    ) -> Result<String, Feedback> {
        if wrapped {
            return Ok(format!("({}{})", self.eval(operator)?, self.eval(node)?));
        }

        Ok(format!("{}{}", self.eval(operator)?, self.eval(node)?))
    }

    fn bin_expr(
        &mut self,
        operator: &Node,
        left: &Node,
        right: &Node,
        wrapped: bool,
    ) -> Result<String, Feedback> {
        if wrapped {
            return Ok(format!(
                "({}{}{})",
                self.eval(left)?,
                self.eval(operator)?,
                self.eval(right)?
            ));
        }

        Ok(format!(
            "{}{}{}",
            self.eval(left)?,
            self.eval(operator)?,
            self.eval(right)?
        ))
    }

    fn field(&mut self, identifier: &Node, field_type: &Node) -> Result<String, Feedback> {
        Ok(format!(
            "{} {}",
            self.eval(field_type)?,
            self.eval(identifier)?
        ))
    }

    fn func_proto(
        &mut self,
        identifier: &Node,
        params: &Vec<Node>,
        return_type: &Node,
    ) -> Result<String, Feedback> {
        let mut res = String::new();

        write!(
            &mut res,
            "{} {}(",
            self.eval(return_type)?,
            self.eval(identifier)?
        )
        .unwrap();

        if !params.is_empty() {
            for param in params {
                write!(&mut res, "{},", &self.eval(param)?).unwrap();
            }

            res.pop();
        } else {
            res.push_str("void");
        }

        res.push_str(");");

        Ok(res)
    }

    fn func_item(&mut self, proto: &Node, body: &Node) -> Result<String, Feedback> {
        let mut res = String::new();

        res.push_str(&self.eval(proto)?);
        res.pop();

        let proto_clone = proto.clone();

        self.func_protos.push(proto_clone);

        res.push('{');
        res.push_str(&self.eval(body)?);
        res.push('}');

        Ok(res)
    }

    fn var_item(
        &mut self,
        identifier: &Node,
        var_type: &Node,
        value: &Option<Node>,
    ) -> Result<String, Feedback> {
        let mut res = String::new();

        write!(res, "{} {}", self.eval(var_type)?, self.eval(identifier)?).unwrap();

        if let Some(value) = value {
            write!(&mut res, "={}", self.eval(value)?).unwrap();
        }

        Ok(res)
    }

    fn type_item(&mut self, identifier: &Node, value: &Node) -> Result<String, Feedback> {
        Ok(format!(
            "typedef {} {};",
            self.eval(value)?,
            self.eval(identifier)?
        ))
    }

    fn struct_item(&mut self, identifier: &Node, fields: &Vec<Node>) -> Result<String, Feedback> {
        let mut res = String::new();
        let eval_identifier = self.eval(identifier)?;

        write!(&mut res, "typedef struct {}{{", eval_identifier).unwrap();

        for field in fields {
            res.push_str(&format!("{};", self.eval(field)?));
        }

        write!(&mut res, "}}{};", eval_identifier).unwrap();

        Ok(res)
    }

    fn assignment(
        &mut self,
        destination: &Node,
        operator: &Node,
        value: &Node,
    ) -> Result<String, Feedback> {
        Ok(format!(
            "{}{}{}",
            self.eval(destination)?,
            self.eval(operator)?,
            self.eval(value)?
        ))
    }

    fn func_call(&mut self, identifier: &Node, params: &Vec<Node>) -> Result<String, Feedback> {
        let mut res = String::new();

        write!(&mut res, "{}(", self.eval(identifier)?).unwrap();

        if !params.is_empty() {
            for node in params {
                write!(&mut res, "{},", self.eval(node)?).unwrap();
            }

            res.pop();
        }

        res.push(')');

        Ok(res)
    }

    fn if_statement(&mut self, condition: &Node, body: &Node) -> Result<String, Feedback> {
        let mut res = String::new();

        write!(&mut res, "if({}){{", self.eval(condition)?).unwrap();
        res.push_str(&self.eval(body)?);
        res.push('}');

        Ok(res)
    }

    fn loop_statement(&mut self, condition: &Node, body: &Node) -> Result<String, Feedback> {
        let mut res = String::new();

        write!(&mut res, "while({}){{", self.eval(condition)?).unwrap();
        res.push_str(&self.eval(body)?);
        res.push('}');

        Ok(res)
    }

    fn return_stmt(&mut self, expr: &Node) -> Result<String, Feedback> {
        Ok(format!("return {}", self.eval(expr)?))
    }

    fn type_node(&mut self, identifier: &Node) -> Result<String, Feedback> {
        self.eval(identifier)
    }

    fn type_ptr_node(&mut self, identifier: &Node, mutable: bool) -> Result<String, Feedback> {
        match mutable {
            true => Ok(format!("{}*", self.eval(identifier)?)),
            false => Ok(format!("const {}*", self.eval(identifier)?)),
        }
    }
}
