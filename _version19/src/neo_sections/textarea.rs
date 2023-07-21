use crate::neo_sections::Section;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

enum TmpAttr {
    Classes(String),
    Cols(u32),
    Id(String),
    // Misc(String),
    Rows(u32),
}

pub fn textarea(source: &str) -> IResult<&str, Section> {
    let (source, _) = tuple((tag_no_case("-- textarea"), not_line_ending, line_ending))(source)?;

    // this is the attrs stuff
    let (source, _) = multispace0(source)?;
    let (source, raw_attrs) = many0(alt((
        delimited(
            tag("-- id: "),
            not_line_ending.map(|x: &str| TmpAttr::Id(x.to_string())),
            line_ending,
        ),
        delimited(
            tag("-- class: "),
            not_line_ending.map(|x: &str| TmpAttr::Classes(x.to_string())),
            line_ending,
        ),
        delimited(
            tag("-- rows: "),
            not_line_ending.map(|x: &str| TmpAttr::Rows(x.parse::<u32>().unwrap())),
            line_ending,
        ),
        delimited(
            tag("-- cols: "),
            not_line_ending.map(|x: &str| TmpAttr::Cols(x.parse::<u32>().unwrap())),
            line_ending,
        ),
    )))(source)?;

    let mut cols: Option<u32> = None;
    let mut classes: Option<Vec<String>> = None;
    let mut id: Option<String> = None;
    let mut rows: Option<u32> = None;

    raw_attrs.iter().for_each(|attr| match attr {
        TmpAttr::Classes(v) => classes = Some(v.split(" ").map(|x| x.to_string()).collect()),
        TmpAttr::Cols(v) => cols = Some(*v),
        TmpAttr::Id(v) => id = Some(v.to_string()),
        TmpAttr::Rows(v) => rows = Some(*v),
        // _ => {}
    });

    let (source, content) = alt((take_until("\n\n--"), rest))(source)?;

    let text = if content.trim().eq("") {
        None
    } else {
        Some(content.trim().to_string())
    };

    Ok((
        source,
        Section::Textarea {
            attributes: None,
            classes,
            cols,
            id,
            text,
            rows,
        },
    ))
}

