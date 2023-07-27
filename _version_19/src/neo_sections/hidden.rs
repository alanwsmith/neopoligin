use nom::branch::alt;
use crate::neo_sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn hidden(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- hidden"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, _) = alt((take_until("\n\n--"), rest))(source.trim())?;
    Ok((source, Section::Hidden))
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::neo_sections::Section;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-- hidden", "", "whiskey tango"].join("\n"), 
        Section::Hidden
    )]
    fn hidden_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, hidden(i.as_str()).unwrap().1)
    }
}
