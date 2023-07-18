use serde::{Deserialize, Serialize};
use nom::branch::alt;
use nom::IResult;
use nom::multi::many0;
use crate::neo_sections::aside::aside;
use crate::attributes::Attribute;
use crate::blocks::Block;

pub mod aside;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoSection {
    Aside{
        attributes: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>
    },
    None
}

// this is split out from neo_section to enable
// direct testing of individual sections
pub fn neo_sections(source: &str) -> IResult<&str, Vec<NeoSection>> {
    let (source, results) = many0(neo_section)(source)?;
    Ok((source, results))
}

pub fn neo_section(source: &str) -> IResult<&str, NeoSection> {
    let (source, results) = 
        alt((
            aside, aside
        ))
    (source)?;
    Ok((source, results))
}