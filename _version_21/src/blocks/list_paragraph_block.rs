use crate::blocks::Block;
use crate::tokens::token;
use nom::bytes::complete::tag;
use nom::combinator::not;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::IResult;

pub fn list_paragraph_block(source: &str) -> IResult<&str, Block, VerboseError<&str>> {
    // seeing a `-` means a new paragraph has started
    let (source, _) = not(tag("-"))(source)?;
    let (source, content) = many1(token)(source)?;
    Ok((source, Block::Paragraph { content }))
}
