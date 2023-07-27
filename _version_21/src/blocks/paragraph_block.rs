use crate::blocks::Block;
use crate::tokens::token;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::IResult;

pub fn paragraph_block(source: &str) -> IResult<&str, Block, VerboseError<&str>> {
    let (source, content) = many1(token)(source)?;
    Ok((source, Block::Paragraph { content }))
}
