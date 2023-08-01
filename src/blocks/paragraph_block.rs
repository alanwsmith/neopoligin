use crate::blocks::Block;
use crate::tokens::token;
// use nom::bytes::complete::tag;
use nom::error::VerboseError;
// use nom::multi::many_till;
use nom::multi::many1;
use nom::IResult;

pub fn paragraph_block(source: &str) -> IResult<&str, Block, VerboseError<&str>> {
    // dbg!(&source);
    let (source, content) = many1(token)(source)?;
    // let (source, content) = many1(token, tag("\n"))(source)?;
    // dbg!(&content);
    // dbg!(&source);
    // Ok((source, Block::Paragraph { content: content.0 }))
    Ok((source, Block::Paragraph { content }))
}
