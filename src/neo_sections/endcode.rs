use crate::sections::alt;
use crate::sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn endcode(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- endcode"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, _) = alt((take_until("\n\n-- "), rest))(source)?;
    Ok((source, Section::None))

    //
}
