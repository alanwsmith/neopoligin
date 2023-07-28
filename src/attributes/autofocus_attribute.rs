use nom::bytes::complete::tag_no_case;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::IResult;
use crate::attributes::*;

pub fn autofocus_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, _attr) = tag_no_case("autofocus")(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::AutoFocus))
}
