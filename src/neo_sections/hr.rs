use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::neo_sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn hr(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- hr"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source.trim())?;
    let (_, attrs) = sec_attrs(content.trim())?;
    Ok((source, Section::Hr { attrs }))
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::section_attrs::SecAttr;
    use crate::neo_sections::Section;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-- hr", "-- class: tango", ""].join("\n"),
        Section::Hr {
            attrs: vec![SecAttr::Class(vec!["tango".to_string()])],
        }
    )]
    fn hr_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, hr(i.as_str()).unwrap().1)
    }
}
