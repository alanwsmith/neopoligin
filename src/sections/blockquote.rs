use crate::blocks::paragraph::paragraph;
use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::sections::Section;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;

pub fn blockquote(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-> blockquote"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n->"), rest))(source.trim())?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (_, paragraphs) = many_till(paragraph, eof)(content.trim())?;
    Ok((
        source,
        Section::Blockquote {
            attrs,
            paragraphs: paragraphs.0,
        },
    ))
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::blocks::Block;
    use crate::sections::Section;
    use crate::tags::Tag;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-> blockquote", "", "whiskey tango"].join("\n"), 
        Section::Blockquote {
            attrs: vec![],
            paragraphs: vec![Block::Paragraph {
                tags: vec![Tag::Text {
                    text: "whiskey tango".to_string(),
                }],
            }],
        }
    )]
    fn blockquote_test(#[case] i: String, #[case] e: Section) {
        assert_eq!(e, blockquote(i.as_str()).unwrap().1)
    }
}
