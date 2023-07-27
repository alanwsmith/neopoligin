use nom::character::complete::line_ending;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;

pub fn spacer_line(source: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (source, _) = pair(line_ending, line_ending)(source)?;
    Ok((source, ""))
}
