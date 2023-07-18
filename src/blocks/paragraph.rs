use crate::blocks::Block;
use crate::snippets::Snippet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::sequence::terminated;
use nom::IResult;
use regex::Regex;

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
