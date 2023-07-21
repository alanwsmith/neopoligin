use crate::section_attrs::sec_attrs;
use nom::branch::alt;
use crate::neo_sections::Section;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

pub fn pre(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- pre"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source.trim())?;
    let (content, _) =
        opt(delimited(tag("-- "), is_not(":\n"), line_ending))(content)?;

    let (content, attrs) = sec_attrs(content.trim())?;
    Ok((
        source,
        Section::Pre {
            attrs,
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
        vec!["-- pre", "", "sierra bravo"].join("\n"), 
        Section::Pre {
            attrs: vec![],
            text: "sierra bravo".to_string()
        }
    )]
    fn pre_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, pre(i.as_str()).unwrap().1)
    }
}
