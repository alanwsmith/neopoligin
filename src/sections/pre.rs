use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::sections::Section;
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
        tuple((tag_no_case("-> pre"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n->"), rest))(source.trim())?;
    let (content, _) =
        opt(delimited(tag(">> "), is_not(":\n"), line_ending))(content)?;

    let (content, attrs) = sec_attrs(content.trim())?;

    // let found_it = attrs.iter_mut().find(|x| match x {
    //     SecAttr::Class(_) => true,
    //     _ => false,
    // });

    // match (found_it, lang) {
    //     (Some(SecAttr::Class(the_thing)), Some(the_lang)) => {
    //         the_thing.push(format!("language-{}", the_lang));
    //     }
    //     (None, Some(the_lang)) => {
    //         attrs.push(SecAttr::Class(vec![format!("language-{}", the_lang)]))
    //     }
    //     _ => {}
    // }
    Ok((
        source,
        Section::Pre {
            attrs,
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
        vec!["-> pre", "", "sierra bravo"].join("\n"), 
        Section::Pre {
            attrs: vec![],
            text: "sierra bravo".to_string()
        }
    )]
    fn pre_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, pre(i.as_str()).unwrap().1)
    }
}
