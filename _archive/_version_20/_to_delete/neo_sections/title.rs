use crate::attrs::attrs;
use crate::blocks::paragraph::paragraph;
use crate::blocks::paragraphs::paragraphs;
use crate::neo_sections::NeoSection;
// use nom::branch::alt;
// use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
// use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
// use nom::combinator::opt;
// use nom::combinator::rest;
// use nom::multi::many0;
// use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
// use nom::Parser;

pub fn title(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- title"),
        not_line_ending,
        newline,
    ))(source)?;
    let (source, attrs) = attrs(source)?;
    let (source, headline) = paragraph(source)?;
    dbg!(&headline);
    dbg!(&source);
    let (source, paragraphs) = paragraphs(source)?;
    dbg!(paragraphs);
    // let (source, blocks) = many_till(paragraph, tag("\n--"))(source)?;
    // let (source, _) =
    //     opt(alt((take_until("\n-- "), rest)).map(|d: &str| d.trim().to_string()))(source)?;
    Ok((
        source,
        NeoSection::Title {
            attrs,
            headline: Some(headline),
            blocks: None,
        },
    ))
}
