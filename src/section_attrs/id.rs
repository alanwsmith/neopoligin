use crate::section_attrs::SecAttr;
use nom::bytes::complete::tag;
use nom::character::complete::not_line_ending;
use nom::sequence::preceded;
use nom::IResult;

pub fn id(source: &str) -> IResult<&str, SecAttr> {
    let (source, captured) = preceded(
        tag(">> id: "),
        not_line_ending,
    )(source.trim())?;
    Ok((
        source,
        SecAttr::Id(
            captured.to_string()
        ),
    ))
}

