use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::sections::Section;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub fn image(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-> image"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n->"), rest))(source)?;
    let (content, id) = opt(preceded(tag(">> "), not_line_ending))(content)?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (_, alt) = alt((take_until("\n\n->"), rest))(content)?;

    Ok((
        source,
        Section::Image {
            alt: alt.to_string(),
            attrs,
            src: id.unwrap().to_string(),
        },
    ))
}

#[cfg(test)]
mod text {
    use super::*;
    // use crate::blocks::Block;
    // use crate::section_attrs::SecAttr;
    use crate::sections::Section;
    // use crate::tags::Tag;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-> image", ">> alfabravo","", "Charlie Delta", "", "-> next"].join("\n"), 
        Ok(("\n\n-> next", 
        Section::Image {
            alt: "Charlie Delta".to_string(),
            attrs: vec![],
            src: "alfabravo".to_string(),
        }))
    )]
    // #[case(
    //     vec!["-> youtube", ">> deltaecho",">> class: foxtrot", "", "whiskey tango", "", "-> next"].join("\n"),
    //     Ok(("\n\n-> next", 
    //     Section::Image {
    //         attrs: vec![
    //             SecAttr::Class(vec!["foxtrot".to_string()])
    //         ],
    //         id: "deltaecho".to_string(),
    //         paragraphs: vec![Block::Paragraph {
    //             tags: vec![Tag::Text {
    //                 text: "whiskey tango".to_string(),
    //             }],
    //         }],
    //     }
    //         ))
    // )]

    fn image_test(#[case] i: String, #[case] e: IResult<&str, Section>) {
        assert_eq!(e, image(i.as_str()))
    }
}
