use crate::contents::Snippet;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::preceded;
use crate::contents::contents;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Paragraph { content: Vec<Snippet> },
    None,
}

pub fn paragraphs(source: &str) -> IResult<&str, Option<Vec<Block>>> {
    let (source, paragraphs) = opt(many1(preceded(multispace0, paragraph)))(source)?;
    Ok((source, paragraphs))
}

pub fn paragraph(source: &str) -> IResult<&str, Block> {
    // seeing a `--` means a new section has started
    let (source, _) = not(tag("--"))(source)?;
    let (source, content) = many1(preceded(opt(line_ending), contents))(source)?;
    Ok((source, Block::Paragraph { content }))
}

pub fn list_paragraph(source: &str) -> IResult<&str, Block> {
    // seeing a `-` means a new paragraph has started
    let (source, _) = not(tag("-"))(source)?;
    let (source, content) = many1(preceded(opt(line_ending), contents))(source)?;
    Ok((source, Block::Paragraph { content }))
}