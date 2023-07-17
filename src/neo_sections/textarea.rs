use crate::section_attrs::sec_attrs;
use nom::branch::alt;
use crate::neo_sections::Section;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::Parser;

pub fn textarea(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- textarea"), not_line_ending, line_ending))(
            source,
        )?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source)?;

    // get the id 
    let (_, id_attr) = opt(
        preceded(
            pair(take_until("-- id: "), tag("-- id: ")),
            not_line_ending
        ).map(|x: &str| x.to_string()))(content)?;

    // get the classes
    let (_, classes) = opt(
        preceded(
            pair(take_until("-- class: "), tag("-- class: ")),
            not_line_ending
        ).map(|x: &str| x.split(" ").into_iter().map(|y| y.to_string()).collect()))(content)?;

        
    let (content, _) =
        opt(delimited(tag("-- "), is_not(":\n"), line_ending))(content)?;
    let (content, attrs) = sec_attrs(content.trim())?;

    if content.eq("") {
        Ok((
            source,
            Section::Textarea {
                attrs: None,
                classes,
                id_attr,
                text: None,
            },
        ))
    }
    else {
        Ok((
            source,
            Section::Textarea {
                attrs: None,
                classes: None,
                id_attr: None,
                text: Some(content.to_string()),
            },
        ))
    }
}