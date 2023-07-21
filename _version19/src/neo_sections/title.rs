use crate::blocks::headline::headline;
use crate::blocks::paragraph::paragraph;
use crate::section_attrs::sec_attrs;
use crate::neo_sections::Section;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag("-- title"), not_line_ending, line_ending))(source.trim())?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source.trim())?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (content, headline) = headline(content.trim())?;
    let (_, paragraphs) = many_till(paragraph, eof)(content.trim())?;

    let result = Section::Title {
        attrs,
        headline,
        paragraphs: paragraphs.0,
    };
    Ok((source, result))
}

#[cfg(test)]

mod test {
    use super::*;
    use crate::blocks::Block;
    use crate::neo_sections::Section;
    use crate::tags::Tag;

    #[test]
    pub fn basic_title() {
        let lines = vec!["-- title", "", "alfa bravo"].join("\n");
        let expected = Section::Title {
            attrs: vec![],
            headline: Block::Headline {
                tags: vec![Tag::Text {
                    text: "alfa bravo".to_string(),
                }],
            },
            paragraphs: vec![],
        };
        assert_eq!(expected, title(lines.as_str()).unwrap().1);
    }

    #[test]
    pub fn title_with_two_lines() {
        let lines =
            vec!["-- title", "", "charlie delta", "echo foxtrot"].join("\n");
        let expected = Section::Title {
            attrs: vec![],
            headline: Block::Headline {
                tags: vec![Tag::Text {
                    text: "charlie delta echo foxtrot".to_string(),
                }],
            },
            paragraphs: vec![],
        };
        assert_eq!(expected, title(lines.as_str()).unwrap().1);
    }

    #[test]
    pub fn title_with_paragraphs() {
        let lines = vec![
            "-- title",
            "",
            "golf hotel",
            "whiskey tango",
            "",
            "foxtrot alfa",
        ]
        .join("\n");
        let expected = Section::Title {
            attrs: vec![],
            headline: Block::Headline {
                tags: vec![Tag::Text {
                    text: "golf hotel whiskey tango".to_string(),
                }],
            },
            paragraphs: vec![Block::Paragraph {
                tags: vec![Tag::Text {
                    text: "foxtrot alfa".to_string(),
                }],
            }],
        };
        assert_eq!(expected, title(lines.as_str()).unwrap().1);
    }
}
