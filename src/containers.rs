use crate::blocks::paragraph;
// use crate::blocks::paragraphs;
use crate::blocks::Block;
use nom::bytes::complete::tag;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use nom::character::complete::multispace0;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem { content: Vec<Block> },
    None,
}

pub fn list_item(source: &str) -> IResult<&str, Container> {
    // let (_, content) = many1(preceded(tag("- "), paragraphs))(source)?;
    // dbg!(&content);
    let (source, content) =preceded(tag("- "), many1(preceded(multispace0, paragraph)))(source)?;
    Ok((source, Container::ListItem { content }))
}


// pub fn paragraphsx(source: &str) -> IResult<&str, Vec<Block>> {
//     let (source, paragraphs) = many1(preceded(multispace0, paragraph))(source)?;
//     Ok((source, paragraphs))
// }

// pub fn paragraph(source: &str) -> IResult<&str, Block> {
//     let (source, _) = not(tag("--"))(source)?;
//     let (source, snippets) = many1(preceded(opt(line_ending), alt((text, strong))))(source)?;
//     Ok((source, Block::Paragraph { snippets }))
// }
