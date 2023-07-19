use crate::containers::Container;
use crate::blocks::Block;
use crate::snippets::Snippet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::sequence::terminated;
use nom::IResult;
use nom::sequence::preceded;
use regex::Regex;

pub fn checklistitem(source: &str) -> IResult<&str, Container> {
    dbg!(&source);
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("[] ")(source)?;
    let (source, captured) = alt((take_until("\n[]"), rest))(source)?;



    // let (source, captured) = preceded(
    //     tag("[] "), 
    //     alt((
    //         terminated(take_until("\n\n"), tag("\n\n")), 
    //         rest))
    //     )(source)?;
    // let re = Regex::new(r"\n").unwrap();
    // let output = re.replace_all(&captured, " ").to_string();
    // dbg!(&output);
    Ok((
        source,
        Container::ChecklistItem {
            blocks: Some(vec![Block::Paragraph {
                snippets: vec![Snippet::Text { text: captured.trim().to_string()}],
            }])
        },
    ))
}
