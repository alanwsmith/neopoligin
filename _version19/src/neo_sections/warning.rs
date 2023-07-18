use crate::blocks::paragraph::paragraph;
use crate::section_attrs::sec_attrs;
use nom::branch::alt;
use crate::neo_sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;

pub fn warning(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- warning"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source.trim())?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (_, paragraphs) = many_till(paragraph, eof)(content.trim())?;
    Ok((
        source,
        Section::Warning {
            attrs,
            paragraphs: paragraphs.0,
        },
    ))
}

// Tests are done in the spec json file
