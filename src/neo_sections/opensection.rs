#![allow(unused_imports)]
use crate::section_attrs::sec_attrs;
use nom::branch::alt;
use crate::neo_sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn opensection(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- opensection"), not_line_ending, line_ending))(source)?;
    let (source, attrs) = sec_attrs(source)?;
    Ok((source, Section::OpenSection { attrs }))
}


#[cfg(test)]
mod text {
    use super::*;
    use crate::section_attrs::SecAttr;
    use crate::neo_sections::Section;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-- opensection", "-- class: alfa", ""].join("\n"),
        Section::OpenSection {
            attrs: vec![SecAttr::Class(vec!["alfa".to_string()])],
        }
    )]
    fn opensection_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, opensection(i.as_str()).unwrap().1)
    }
}
