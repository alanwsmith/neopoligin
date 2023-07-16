use crate::neo_sections::Section;
use crate::section_attrs::sec_attrs;
use crate::section_attrs::SecAttr;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::anychar;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub fn code(source: &str) -> IResult<&str, Section> {
    let (source, _) = tuple((tag_no_case("-- code"), not_line_ending, line_ending))(source.trim())?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source.trim())?;

    let (content, lang) = opt(delimited(tag("-- "), is_not(":\n"), line_ending))(content.trim())?;

    let (content, mut attrs) = sec_attrs(content.trim())?;
    // let (content, _) = preceded(multispace0, eof)(content.trim())?;

    let found_it = attrs.iter_mut().find(|x| match x {
        SecAttr::Class(_) => true,
        _ => false,
    });

    match (found_it, lang) {
        (Some(SecAttr::Class(the_thing)), Some(the_lang)) => {
            the_thing.push(format!("language-{}", the_lang));
        }
        (None, Some(the_lang)) => {
            attrs.push(SecAttr::Class(vec![format!("language-{}", the_lang)]))
        }
        _ => {}
    }

    // dbg!("--------------");
    // dbg!(&content);
    // dbg!(&content.trim());
    // dbg!("--------------");

    Ok((
        source,
        Section::Code {
            attrs,
            text: content.trim().to_string(),
        },
    ))

    //
}

#[cfg(test)]
mod text {
    use super::*;
    // use crate::blocks::Block;
    use crate::neo_sections::Section;
    // use crate::tags::Tag;
    use crate::section_attrs::SecAttr;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-- code", "", "sierra bravo"].join("\n"), 
        Section::Code {
            attrs: vec![],
            text: "sierra bravo".to_string()
        }
    )]
    #[case(
        vec!["-- code", "-- rust", "", "echo foxtrot"].join("\n"), 
        Section::Code {
            attrs: vec![SecAttr::Class(vec!["language-rust".to_string()])],
            text: "echo foxtrot".to_string()
        }
    )]
    fn code_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, code(i.as_str()).unwrap().1)
    }
}
