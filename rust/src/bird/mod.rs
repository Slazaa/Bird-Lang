pub mod compile;
pub mod constants;
pub mod feedback;
pub mod lexer;
pub mod parser;

use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use self::compile::c;
use self::feedback::*;
use self::lexer::*;
use self::parser::*;

pub static SRC_FOLDER: &str = "src";

enum PathOrFile {
    Path(PathBuf),
    Filename(String),
}

/// The `c` mode compiles the code into C.
pub fn to_c(output: &str) -> Result<(), Feedback> {
    for file in WalkDir::new(SRC_FOLDER).into_iter().filter_map(|x| x.ok()) {
        if !file.metadata().unwrap().is_file() {
            continue;
        }

        filename_to_c(file.path(), output)?;
    }

    //text_to_c(&c::array::array(), PathOrFile::Filename("array.bird".to_owned()))?;

    Ok(())
}

fn filename_to_c(file_path: &Path, output: &str) -> Result<(), Feedback> {
    let text = match fs::read_to_string(file_path) {
        Ok(x) => x,
        Err(_) => return Err(Error::no_file_or_dir(file_path.to_str().unwrap())),
    };

    text_to_c(&text, PathOrFile::Path(file_path.to_path_buf()), output)
}

fn text_to_c(text: &str, path_or_file: PathOrFile, output: &str) -> Result<(), Feedback> {
    let tokens = match &path_or_file {
        PathOrFile::Path(path) => Lexer::parse(text, Some(path))?,
        PathOrFile::Filename(_) => Lexer::parse(text, None)?,
    };

    let ast = Parser::parse(&tokens)?;

    match &path_or_file {
        PathOrFile::Path(path) => c::Compiler::compile(&ast, path, output)?,
        PathOrFile::Filename(filename) => {
            c::Compiler::compile(&ast, &Path::new("bird/{}").join(filename), output)?
        }
    }

    Ok(())
}
