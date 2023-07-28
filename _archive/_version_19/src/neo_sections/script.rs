// use crate::section_attrs::sec_attrs;
use nom::branch::alt;
use crate::neo_sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn script(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- script"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source.trim())?;
    Ok((
        source,
        Section::Script {
            text: content.to_string(),
        },
    ))
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::neo_sections::Section;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-- script", "", "const widget = `alfa`"].join("\n"), 
        Section::Script {
            text: "const widget = `alfa`".to_string()
        }
    )]
    fn html_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, script(i.as_str()).unwrap().1)
    }
}
