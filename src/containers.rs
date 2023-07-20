use crate::blocks::paragraph;
use crate::blocks::Block;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem { content: Vec<Block> },
    None,
}

pub fn list_item(source: &str) -> IResult<&str, Container> {
    let (source, content) = many1(preceded(tag("- "), paragraph))(source)?;
    Ok((source, Container::ListItem { content }))
}
