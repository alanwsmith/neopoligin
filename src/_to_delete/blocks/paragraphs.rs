use crate::blocks::Block;
use crate::blocks::paragraph::paragraph;
use nom::combinator::opt;
use nom::multi::many1;

// use crate::snippets::Snippet;
// use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::bytes::complete::take_until;
// use nom::character::complete::multispace0;
// use nom::combinator::rest;
// use nom::sequence::terminated;
use nom::IResult;
// use regex::Regex;

pub fn paragraphs(source: &str) -> IResult<&str, Option<Vec<Block>>> {
    dbg!(&source);
    let (source, paragraphs) = many1(paragraph)(source)?;
    dbg!(&paragraphs);
    dbg!(&source);
    Ok((
        source,
        Some(paragraphs)
    ))
}
