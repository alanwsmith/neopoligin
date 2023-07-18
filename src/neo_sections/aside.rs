#![allow(unused_imports)]
use crate::neo_sections::Attribute;
use crate::neo_sections::Block;
use crate::neo_sections::NeoSection;
use crate::snippets::Snippet;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::crlf;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::none_of;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn aside(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- aside"),
        not_line_ending,
        newline,
    ))(source)?;

    // get attributes here
    dbg!("1111111111111111111111111111");
    dbg!(&source);
    let (source, paragraphs) = opt(many0(paragraph))(source)?;
    dbg!("5555555555555555555555555555");
    dbg!(&paragraphs);
    dbg!(&source);

    Ok((
        source,
        NeoSection::Aside {
            attributes: None,
            blocks: paragraphs,
        },
    ))
}

pub fn paragraph(source: &str) -> IResult<&str, Block> {
    dbg!("222222222222222222222222222");
    dbg!(&source);
    let (source, snippets) = many_till(
        snippet,
        alt((
            tuple((space0, crlf, space0, crlf)).map(|_| ""),
            pair(multispace0, eof).map(|_| ""),
        )),
    )(source)?;
    dbg!("444444444444444444444444444444");
    dbg!(&snippets);
    dbg!(&source);
    Ok((
        source,
        Block::Paragraph {
            snippets: snippets.0,
        },
    ))
    // Ok((source, Block::None))
}

pub fn snippet(source: &str) -> IResult<&str, Snippet> {
    dbg!("3333333333333333333333333333");
    let (source, _) = multispace0(source)?;
    let (source, value) = many_till(
        alt((
            is_not("\n\r\t"),
            pair(line_ending, not(crlf)).map(|_| " "),
            rest,
        )),
        alt((
            eof.map(|_| ""),
            tuple((line_ending, space0, line_ending)).map(|_| ""),
            eof,
        )),
    )(source)?;
    dbg!(&value);
    dbg!(&source);

    Ok((
        source,
        Snippet::Text {
            text: value.0.join(""),
            // text: "asdfasdfasdfasdfdadfs".to_string(),
        },
    ))
}

// pub fn word(source: &str) -> IResult<&str, &str> {
//     let (source, word) = is_not(" \n\r\t")(source)?;
//     Ok((source, word))
// }

// pub fn not_separator(source: &str) -> IResult<&str, &str> {
//     let (source, _) =
//     Ok((source, ""))
// }

// pub fn separator(source: &str) -> IResult<&str, Snippet> {
//     let

// }
