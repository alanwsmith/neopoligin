use crate::attributes::AttributesObj;
use nom::branch::alt;
use crate::neo_sections::p_section::p_section;
use serde::{Deserialize, Serialize};
use crate::blocks::Block;
use crate::containers::Container;

pub mod aside_section;
pub mod h1_section;
pub mod h2_section;
pub mod h3_section;
pub mod h4_section;
pub mod h5_section;
pub mod h6_section;
pub mod image_section;
pub mod list_section;
pub mod p_section;
pub mod title_section;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoSection {
    Aside {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
    },
    H1 {
        attributes: Option<AttributesObj>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H2 {
        attributes: Option<AttributesObj>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H3 {
        attributes: Option<AttributesObj>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H4 {
        attributes: Option<AttributesObj>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H5 {
        attributes: Option<AttributesObj>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H6 {
        attributes: Option<AttributesObj>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    Image {
        attributes: Option<AttributesObj>,
        caption: Option<Vec<Block>>,
        src: Option<String>,
    },
    List {
        attributes: Option<AttributesObj>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    Title {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    P {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
    },
    RawPageAttributes(Vec<(String, String)>),
}

pub fn neo_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, section) = alt((
        aside_section,
        image_section,
        h1_section,
        h2_section,
        h3_section,
        h4_section,
        h5_section,
        h6_section,
        list_section,
        p_section,
        title_section,
    ))(source)?;
    Ok((source, section))
}
