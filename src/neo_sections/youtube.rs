use crate::blocks::paragraph::paragraph;
use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::sections::Section;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub fn youtube(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- youtube"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source)?;
    let (content, id) = opt(preceded(tag("-- "), not_line_ending))(content)?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (_, paragraphs) = many_till(paragraph, eof)(content.trim())?;

    Ok((
        source,
        Section::Youtube {
            attrs,
            id: id.unwrap().to_string(),
            paragraphs: paragraphs.0,
        },
    ))
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::blocks::Block;
    use crate::section_attrs::SecAttr;
    use crate::sections::Section;
    use crate::tags::Tag;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-- youtube", "-- alfabravo","", "-- next"].join("\n"), 
        Ok(("\n\n-- next", 
        Section::Youtube {
            attrs: vec![],
            id: "alfabravo".to_string(),
            paragraphs: vec![]

        }))
    )]
    #[case(
        vec!["-- youtube", "-- deltaecho","-- class: foxtrot", "", "whiskey tango", "", "-- next"].join("\n"),
        Ok(("\n\n-- next", 
        Section::Youtube {
            attrs: vec![
                SecAttr::Class(vec!["foxtrot".to_string()])
            ],
            id: "deltaecho".to_string(),
            paragraphs: vec![Block::Paragraph {
                tags: vec![Tag::Text {
                    text: "whiskey tango".to_string(),
                }],
            }],
        }
            ))
    )]

    fn youtube_test(#[case] i: String, #[case] e: IResult<&str, Section>) {
        assert_eq!(e, youtube(i.as_str()))
    }
}
