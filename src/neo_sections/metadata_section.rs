use crate::neo_sections::NeoSection;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "t", content = "value", rename_all = "lowercase")]
pub enum MetadataItem {
    Date(String),
    Id(String),
    Path(String),
    Status(String),
    Template(String),
    Type(String),
    Generic { name: String, string: String },
}

pub fn metadata_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- metadata")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, list) = opt(many1(
        alt((
            date_metadata,
            id_metadata,
            path_metadata,
            status_metadata,
            template_metadata,
            type_metadata,
            generic_metadata,
        )), // metadata_item
    ))(source)?;
    Ok((source, NeoSection::Metadata { list }))
}

pub fn date_metadata(source: &str) -> IResult<&str, MetadataItem, VerboseError<&str>> {
    let (source, value) =
        delimited(pair(tag("-- date:"), space1), not_line_ending, line_ending)(source)?;
    Ok((source, MetadataItem::Date(value.to_string())))
}

pub fn generic_metadata(source: &str) -> IResult<&str, MetadataItem, VerboseError<&str>> {
    let (source, name) =
        delimited(pair(tag("--"), space1), is_not(":"), pair(tag(":"), space0))(source)?;
    let (source, value) = terminated(not_line_ending, line_ending)(source)?;

    Ok((
        source,
        MetadataItem::Generic {
            name: name.trim().to_string(),
            string: value.trim().to_string(),
        },
    ))
}

pub fn id_metadata(source: &str) -> IResult<&str, MetadataItem, VerboseError<&str>> {
    let (source, value) =
        delimited(pair(tag("-- id:"), space1), not_line_ending, line_ending)(source)?;
    Ok((source, MetadataItem::Id(value.to_string())))
}

pub fn path_metadata(source: &str) -> IResult<&str, MetadataItem, VerboseError<&str>> {
    let (source, value) =
        delimited(pair(tag("-- path:"), space1), not_line_ending, line_ending)(source)?;
    Ok((source, MetadataItem::Path(value.to_string())))
}

pub fn status_metadata(source: &str) -> IResult<&str, MetadataItem, VerboseError<&str>> {
    let (source, value) = delimited(
        pair(tag("-- status:"), space1),
        not_line_ending,
        line_ending,
    )(source)?;
    Ok((source, MetadataItem::Status(value.to_string())))
}

pub fn template_metadata(source: &str) -> IResult<&str, MetadataItem, VerboseError<&str>> {
    let (source, value) = delimited(
        pair(tag("-- template:"), space1),
        not_line_ending,
        line_ending,
    )(source)?;
    Ok((source, MetadataItem::Template(value.to_string())))
}

pub fn type_metadata(source: &str) -> IResult<&str, MetadataItem, VerboseError<&str>> {
    let (source, value) =
        delimited(pair(tag("-- type:"), space1), not_line_ending, line_ending)(source)?;
    Ok((source, MetadataItem::Type(value.to_string())))
}
