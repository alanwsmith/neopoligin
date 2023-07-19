use serde::{Deserialize, Serialize};
use nom::branch::alt;
use nom::IResult;
use nom::multi::many0;
pub mod aside;
use crate::neo_sections::aside::aside;
pub mod attributes;
use crate::neo_sections::attributes::attributes;
pub mod audio;
use crate::neo_sections::audio::audio;
pub mod blockquote;
use crate::neo_sections::blockquote::blockquote;
pub mod blurb;
use crate::neo_sections::blurb::blurb;
pub mod canvas;
use crate::neo_sections::canvas::canvas;
pub mod categories;
use crate::neo_sections::categories::categories;
pub mod checklist;
use crate::neo_sections::checklist::checklist;
use crate::attrs::Attribute;
use crate::blocks::Block;
use crate::containers::Container;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoSection {
    Aside {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>
    },
    Attributes {
        attrs: Option<Vec<Attribute>>,
    },
    Audio {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>
    },
    Blockquote {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>
    },
    Blurb {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>
    },
    Canvas {
        attrs: Option<Vec<Attribute>>
    },
    Categories {
        list: Vec<String>
    },
    Checklist { 
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>,
        items: Option<Vec<Container>>
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
            aside, attributes, audio, blockquote, blurb, canvas, categories, 
            checklist
        ))
    (source)?;
    Ok((source, results))
}