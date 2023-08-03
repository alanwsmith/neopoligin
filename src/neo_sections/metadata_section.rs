use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::pair;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::bytes::complete::is_not;
use nom::IResult;
use nom::sequence::preceded;
use serde::{Deserialize, Serialize};

pub fn metadata_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- metadata")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, list) = opt(many1(metadata_item))(source)?;
    Ok((
        source,
        NeoSection::Metadata {
            list,
        },
    ))
}

pub fn metadata_item(source: &str) -> IResult<&str, MetadataItem, VerboseError<&str>> {
    let (source, key) = preceded(tag("-- "), is_not(":"))(source)?;
    let (source, _) = pair(tag(":"), space1)(source)?;
    let (source, value) = not_line_ending(source)?;
    let (source, _) = line_ending(source)?; 
    Ok((source, MetadataItem::Generic{key: key.to_string(), value: value.to_string()}))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", rename_all = "lowercase")]
pub enum MetadataItem {
    Generic{ key: String, value: String}
}