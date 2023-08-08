use crate::blocks::Block;
use crate::tokens::token;
// use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::IResult;
use nom::combinator::not;
use nom::bytes::complete::tag;
use nom::branch::alt;

pub fn paragraph_block(source: &str) -> IResult<&str, Block, VerboseError<&str>> {
    // dbg!(&source);
    // let (source, _) = multispace0(source)?;
    // dbg!(&source);
    let (source, _) = not(alt((tag("-"), tag("["))))(source)?;
    let (source, content) = many1(token)(source)?;
    // dbg!(&content);
    // dbg!(&source);
    Ok((source, Block::Paragraph { content }))
}
