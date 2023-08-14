use crate::blocks::todo_paragraph_block::todo_paragraph_block;
use crate::containers::Container;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::IResult;
use nom::character::complete::multispace0;
use nom::character::complete::line_ending;
use nom::sequence::terminated;
use nom::character::complete::space1;
// use nom::bytes::complete::is_not;
// use nom::combinator::opt;
// use nom::Parser;

pub fn checklist_item_container(source: &str) -> IResult<&str, Container, VerboseError<&str>> {
    // dbg!(&source);
    let (source, _) = multispace0(source)?;
    // dbg!(&source);
    let (source, _) = tag("[]")(source)?;
    let (source, _) = space1(source)?;
    // dbg!(&source);
    let (source, body) = many1(terminated(todo_paragraph_block, line_ending))(source)?;
    // dbg!(&body);
    // dbg!(&source);
    Ok((source, Container::ChecklistItem { body }))
}
