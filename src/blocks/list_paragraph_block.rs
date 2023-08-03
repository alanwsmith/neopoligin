// use crate::blocks::Block;
// use crate::tokens::token;
// use nom::bytes::complete::tag;
// // use nom::combinator::not;
// use nom::error::VerboseError;
// use nom::multi::many1;
// use nom::IResult;
// use nom::sequence::preceded;
// use nom::combinator::not;

// pub fn list_paragraph_block(source: &str) -> IResult<&str, Block, VerboseError<&str>> {
//     dbg!(source);
//     let (source, content) = many1(preceded(not(tag("\n-")), token))(source)?;
//     dbg!(&content);
//     dbg!(&source);
//     Ok((source, Block::Paragraph { content }))
// }
