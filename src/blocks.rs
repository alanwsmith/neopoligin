use crate::snippets::strong;
use crate::snippets::text;
use crate::snippets::Snippet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Paragraph { snippets: Vec<Snippet> },
    None,
}

pub fn paragraphs(source: &str) -> IResult<&str, Option<Vec<Block>>> {
    let (source, paragraphs) = opt(many1(preceded(multispace0, paragraph)))(source)?;
    Ok((source, paragraphs))
}

pub fn paragraph(source: &str) -> IResult<&str, Block> {
    let (source, _) = not(tag("--"))(source)?;
    let (source, snippets) = many1(preceded(opt(line_ending), alt((text, strong))))(source)?;
    Ok((source, Block::Paragraph { snippets }))
}
