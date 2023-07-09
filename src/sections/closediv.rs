use crate::sections::alt;
use crate::sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn closediv(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-> closediv"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, _) = alt((take_until("\n\n->"), rest))(source.trim())?;
    //let (_, attrs) = sec_attrs(content.trim())?;
    Ok((source, Section::CloseDiv))
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::sections::Section;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-> closediv", "", "-> ect"].join("\n"),
        Section::CloseDiv
    )]
    fn closediv_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, closediv(i.as_str()).unwrap().1)
    }
}
