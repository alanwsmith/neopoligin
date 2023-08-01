




use nom::character::complete::line_ending;




use nom::character::complete::space0;




use nom::error::VerboseError;





use nom::sequence::pair;


use nom::IResult;






pub fn empty_line(source: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (source, _) = pair(space0, line_ending)(source)?;
    Ok((source, ""))
}
