use crate::neo_sections::Block;
use crate::neo_sections::NeoSection;
use crate::snippets::Snippet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use regex::Regex;

pub fn aside(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- aside"),
        not_line_ending,
        newline,
    ))(source)?;
    let (source, paragraphs) = many_till(paragraph, eof)(source)?;
    Ok((
        source,
        NeoSection::Aside {
            attributes: None,
            blocks: Some(paragraphs.0),
        },
    ))
}

pub fn paragraph(source: &str) -> IResult<&str, Block> {
    let (source, _) = multispace0(source)?;
    let (source, captured) = alt((terminated(take_until("\n\n"), tag("\n\n")), rest))(source)?;
    let re = Regex::new(r"\n").unwrap();
    let output = re.replace_all(&captured, " ").to_string();
    Ok((
        source,
        Block::Paragraph {
            snippets: vec![Snippet::Text { text: output }],
        },
    ))
}

// pub fn block_end(source: &str) -> IResult<&str, &str> {
//     let (source, remainder) = eof(source)?;
//     Ok((source, remainder))
// }

// pub fn section_end(source: &str) -> IResult<&str, &str> {
//     let (source, remainder) = eof(source)?;
//     Ok((source, remainder))
// }

// pub fn paragraphs(source: &str) -> IResult<&str, Option<Vec<Block>>> {
//     dbg!("77777777777777");
//     let (source, _) = multispace0(source)?;
//     dbg!("88888888888888");
//     let (source, graphs) = opt(many_till(paragraph, eof))(source)?;
//     dbg!("99999999999999");
//     dbg!(&graphs);
//     Ok((source, Some(graphs.unwrap().0)))
// }

// pub fn snippet(source: &str) -> IResult<&str, Snippet> {
//     dbg!(&source);
//     let (source, text) = many_till(
//         anychar.map(|x| if x == '\n' { ' ' } else { x }),
//         alt((tuple((line_ending, line_ending)).map(|_| ""), eof)),
//     )(source)?;
//     let response1 = &text.0.clone().into_iter().collect::<String>();
//     dbg!(&response1);
//     Ok((
//         source,
//         Snippet::Text {
//             text: response1.to_string(),
//         },
//     ))
// }

// pub fn block_separator(source: &str) -> IResult<&str, Snippet> {
//     let (source, _) = alt((tuple((line_ending, line_ending)).map(|_| ""), eof))(source)?;
//     dbg!(&response1);
//     Ok((
//         source,
//         Snippet::Text {
//             text: response1.to_string(),
//         },
//     ))
// }

// pub fn paragraph(source: &str) -> IResult<&str, Block> {
//     let (source, _) = multispace0(source)?;
//     dbg!("00000000000000");
//     let (source, snippets) = many_till(
//         snippet,
//         alt((tuple((line_ending, line_ending)).map(|_| ""), eof)),
//     )(source)?;
//     dbg!("22222222222222");
//     dbg!(&snippets);
//     dbg!(&source);
//     Ok((
//         source,
//         Block::Paragraph {
//             snippets: snippets.0,
//         },
//     ))
//     // let (source, snippets) = many_till(
//     //     snippet,
//     //     alt((
//     //         tuple((space0, crlf, space0, crlf)).map(|_| ""),
//     //         pair(multispace0, eof).map(|_| ""),
//     //         rest,
//     //     )),
//     // )(source)?;
//     // dbg!("44444444444");
//     // dbg!(&snippets);
//     // dbg!(&source);
//     // Ok((
//     //     source,
//     //     // Block::Paragraph {
//     //     //     snippets: snippets.0,
//     //     // },
//     //     Block::None,
//     // ))
// }

// pub fn snippets(source: &str) -> IResult<&str, Snippet> {
//     let (source, _) = multispace0(source)?;
//     Ok((source, Snippet::None))
// }

// pub fn snippet(source: &str) -> IResult<&str, Snippet> {
//     dbg!("33333333333");
//     let (source, _) = multispace0(source)?;
//     let (source, value) = many_till(
//         alt((
//             is_not("\n\r\t"),
//             pair(line_ending, not(crlf)).map(|_| " "),
//             rest,
//         )),
//         alt((
//             eof.map(|_| ""),
//             tuple((line_ending, space0, line_ending)).map(|_| ""),
//             eof,
//         )),
//     )(source)?;
//     dbg!(&value);
//     dbg!(&source);
//     Ok((
//         source,
//         Snippet::Text {
//             text: value.0.join(""),
//             // text: "asdfasdfasdfasdfdadfs".to_string(),
//         },
//     ))
// }

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
