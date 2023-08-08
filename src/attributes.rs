use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::sequence::terminated;
use nom::Parser;
use nom::combinator::not;

use crate::neo_sections::neo_section;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum AttributeV2 {
    AccessKey(String),
    AutoCapitalize(String),
    AutoFocus,
    ContentEditable(String),
    Class(Vec<String>),
    Generic((String, String)),
    Hidden,
    Id(String),
    Show,
    Subtitle(String),
    Title(String),
    Type(String),
    Url(String),
    None,
}


pub fn attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    // dbg!(&source);
    // because lines are flattened you have to make sure something that
    // starts with two dashes isn't a neo section before looking for
    // attributes
    let (source, _) = not(neo_section)(source)?;
    let (source, attrs) = preceded(
        alt((tag("--"), tag("|"))),
        alt((
            hidden_attribute,
            accesskey_attribute,
            autocapitalize_attribute,
            autofocus_attribute,
            class_attribute,
            contenteditable_attribute,
            id_attribute,
            show_attribute,
            subtitle_attribute,
            title_attribute,
            type_attribute,
            url_attribute,
            // generic must be last
            generic_attribute,
        )),
    )(source)?;
    Ok((source, attrs))
}


pub fn accesskey_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("accesskey: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::AccessKey(attr.trim().to_string())))
}


pub fn autocapitalize_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    // autocapitzlize has a specific set of options but the
    // ROI of limiting to them isn't high for phase 1 so
    // just passing in whatever string. Something to look into
    // for a future iteration maybe
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("autocapitalize: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::AutoCapitalize(attr.trim().to_string())))
}


pub fn autofocus_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, _attr) = tag_no_case("autofocus")(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::AutoFocus))
}


pub fn class_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, _attr) = tag("class:")(source)?;
    let (source, value) = many1(preceded(
        space0,
        is_not(" |>\n").map(|c: &str| c.trim().to_string()),
    ))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Class(value)))
}


pub fn contenteditable_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, value) = preceded(tag("contenteditable: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::ContentEditable(value.trim().to_string())))
}


pub fn generic_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, key) = terminated(is_not(":"), tag(":"))(source)?;
    let (source, _) = space0(source)?;
    let (source, value) = is_not("|>\n")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Generic((key.trim().to_string(), value.trim().to_string()))))
}

pub fn hidden_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    dbg!(&source);
    let (source, _) = space0(source)?;
    dbg!(&source);
    let (source, _attr) = tag_no_case("hidden")(source)?;
    dbg!(&source);
    let (source, _) = opt(line_ending)(source)?;
    dbg!(&source);
    Ok((source, AttributeV2::Hidden))
}

pub fn id_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("id: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Id(attr.trim().to_string())))
}


pub fn show_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, _attr) = tag_no_case("show")(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Show))
}

pub fn subtitle_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("subtitle: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Subtitle(attr.trim().to_string())))
}


pub fn title_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("title: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Title(attr.trim().to_string())))
}


pub fn type_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("type: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Type(attr.trim().to_string())))
}


pub fn url_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("url: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Url(attr.trim().to_string())))
}
