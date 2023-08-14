use crate::blocks::paragraph_block::paragraph_block;
use crate::tokens::Token;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};

pub mod list_paragraph_block;
pub mod paragraph_block;
pub mod todo_paragraph_block;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Paragraph { content: Vec<Token> },
    RawString { content: String },
}

pub fn block(source: &str) -> IResult<&str, Block, VerboseError<&str>> {
    // dbg!(source);
    let (source, _) = not(pair(multispace0, tag("--")))(source)?;
    // dbg!(source);
    let (source, block) = preceded(multispace0, paragraph_block)(source)?;
    // dbg!(source);
    Ok((source, block))
}
