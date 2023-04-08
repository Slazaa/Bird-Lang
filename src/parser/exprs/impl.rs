use nom::{
    IResult, Parser,
    sequence::tuple
};
use nom_supreme::{
    ParserExt,
    error::ErrorTree,
    tag::complete::tag
};

use super::{ws, r#type::Type, block::Block};

#[derive(Debug)]
pub struct Impl<'a> {
    pub r#type: Type<'a>,
    pub body: Block<'a>
}

impl<'a> Impl<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
        tuple((
            ws(Type::parse).preceded_by(tag("impl")),
            ws(Block::parse)
        ))
            .parse(input)
            .map(|(input, (r#type, body))| {
                (input, Self { r#type, body })
            })
    }
}
