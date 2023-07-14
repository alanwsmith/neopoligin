use crate::section_attrs::sec_attrs;
use nom::branch::alt;
use crate::section_attrs::SecAttr;
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

pub fn startcode(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- startcode"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n-- endcode"), rest))(source.trim())?;
    
    let (content, lang) =
        opt(delimited(tag("-- "), is_not(":\n"), line_ending))(content)?;

    let (content, mut attrs) = sec_attrs(content.trim())?;

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
    Ok((
        source,
        Section::Code {
            attrs,
            text: content.to_string(),
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
        vec!["-- startcode", "", "kick it", "", "-- h2", "", "-- endcode"].join("\n"), 
        Section::Code {
            attrs: vec![],
            text: "kick it\n\n-- h2".to_string()
        }
    )]
    #[case(
        vec!["-- startcode", "-- rust", "", "kick it", "", "-- h2", "", "-- endcode"].join("\n"), 
        Section::Code {
            attrs: vec![SecAttr::Class(vec!["language-rust".to_string()])],
            text: "kick it\n\n-- h2".to_string()
        }
    )]
    fn solo_code_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, startcode(i.as_str()).unwrap().1)
    }
}
