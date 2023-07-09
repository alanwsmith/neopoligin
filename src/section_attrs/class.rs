use crate::section_attrs::SecAttr;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

pub fn class(source: &str) -> IResult<&str, SecAttr> {
    let (source, captured) = preceded(
        tag(">> class: "),
        not_line_ending,
    )(source.trim())?;
    let (_, classes) = separated_list1(
        space1,
        is_not(" "),
    )(captured.trim())?;
    Ok((
        source,
        SecAttr::Class(
            classes.iter().map(|s| s.to_string()).collect(),
        ),
    ))
}

