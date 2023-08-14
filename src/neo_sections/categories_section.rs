use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::pair;
use nom::character::complete::not_line_ending;
use nom::IResult;
use nom::combinator::not;
use nom::sequence::preceded;
use crate::neo_sections::neo_section;

pub fn categories_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- categories")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, list) = opt(many1(category))(source)?;
    Ok((
        source,
        NeoSection::Categories {
            list,
        },
    ))
}

fn category(source: &str) -> IResult<&str, String, VerboseError<&str>> {
    let (source, _) = not(neo_section)(source)?;
    let (source, cat) = preceded(tag("-- "), not_line_ending)(source)?;
    let (source, _) = line_ending(source)?;
    Ok((source, cat.to_string()))
}

