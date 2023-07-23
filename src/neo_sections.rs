#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
use crate::attributes::attributes;
use crate::attributes::AttributesObj;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::error::VerboseErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::blocks::Block;
use crate::containers::Container;
use crate::neo_sections::aside_section::aside_section;
use crate::neo_sections::image_section::image_section;
use crate::neo_sections::list_section::list_section;
use crate::neo_sections::p_section::p_section;
use crate::neo_sections::title_section::title_section;

pub mod aside_section;
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
        list_section,
        p_section,
        title_section,
    ))(source)?;
    Ok((source, section))
}
