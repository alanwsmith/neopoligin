use crate::blocks::list_paragraph;
use crate::blocks::Block;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use nom::character::complete::multispace0;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem { content: Vec<Block> },
    None,
}

pub fn list_item(source: &str) -> IResult<&str, Container> {
    let (source, _) = tag("- ")(source)?;
    let (source, content) = many1(preceded(multispace0, list_paragraph))(source)?;
    Ok((source, Container::ListItem { content }))
}

