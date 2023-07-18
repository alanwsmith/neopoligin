use crate::attributes::Attribute;
use crate::neo_sections::Block;
use crate::neo_sections::NeoSection;
use crate::snippets::Snippet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use regex::Regex;

pub fn attribute(source: &str) -> IResult<&str, Attribute> {
    let (source, attr) = delimited(tag("-- accesskey: "), not_line_ending, line_ending)(source)?;
    Ok((source, Attribute::AccessKey(attr.to_string())))
}

pub fn attributes(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (source, attributes) = many0(attribute)(source)?;
    if attributes.len() == 0 {
        Ok((source, None))
    } else {
        Ok((source, Some(attributes)))
    }
}

pub fn aside(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- aside"),
        not_line_ending,
        newline,
    ))(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, paragraphs) = many_till(paragraph, eof)(source)?;
    Ok((
        source,
        NeoSection::Aside {
            attributes,
            blocks: Some(paragraphs.0),
        },
    ))
}

pub fn paragraph(source: &str) -> IResult<&str, Block> {
    let (source, _) = multispace0(source)?;
    let (source, captured) = alt((terminated(take_until("\n\n"), tag("\n\n")), rest))(source)?;
    let re = Regex::new(r"\n").unwrap();
    let output = re.replace_all(&captured, " ").to_string();
    Ok((
        source,
        Block::Paragraph {
            snippets: vec![Snippet::Text { text: output }],
        },
    ))
}
