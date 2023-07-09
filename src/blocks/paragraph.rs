use crate::blocks::Block;
use crate::tags::tags;
// use crate::snippets::Snippet;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;

pub fn paragraph(source: &str) -> IResult<&str, Block> {
    let (source, content) = many_till(
        pair(not_line_ending, alt((line_ending, eof))).map(|x| x.0),
        alt((multispace1, eof)),
    )(source.trim())?;
    let string = content.0.join(" ");
    let (_, tags) = tags(string.as_str()).unwrap();
    Ok((source, Block::Paragraph { tags }))
}
