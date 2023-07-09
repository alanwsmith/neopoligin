// use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;

pub fn html(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-> html"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n->"), rest))(source.trim())?;

    Ok((
        source,
        Section::Html {
            text: content.to_string(),
        },
    ))

    //
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::sections::Section;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-> html", "", "sierra bravo"].join("\n"), 
        Section::Html {
            text: "sierra bravo".to_string()
        }
    )]
    fn html_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, html(i.as_str()).unwrap().1)
    }
}
