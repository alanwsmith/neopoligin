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

use crate::blocks::block;
use crate::containers::list_item_container::list_item_container;
use crate::helpers::empty_line::empty_line;
use crate::neo_sections::NeoSection;

pub fn list_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    // Version 1 tests in place
    let (source, _) = tag_no_case("-- list")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = opt(attributes)(source)?;
    let (source, _) = empty_line(source)?;
    // let (source, preface) = opt(block)(source)?;
    let (source, items) = opt(many1(preceded(multispace0, list_item_container)))(source)?;

    // let (source, _) = line_ending(source)?;
    // let (source, attributes) = attributes(source)?;
    // let (source, preface) = opt(many1(preceded(multispace0, list_paragraph)))(source)?;
    // let (source, items) = opt(many1(preceded(multispace0, list_item)))(source)?;
    Ok((
        source,
        NeoSection::List {
            attributes,
            items,
            preface: None,
        },
    ))
}
