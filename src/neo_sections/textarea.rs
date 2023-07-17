use crate::neo_sections::Section;
use crate::section_attrs::SecAttr;
use nom::branch::alt;
// use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::combinator::rest;
// use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
// use nom::combinator::recognize;
// use nom::multi::many_till;
use nom::multi::many0;

//fn attr_id(source: &str) -> IResult<&str, SecAttr> {
//    dbg!("33333333333333333333333333333333");
//    let (source, attr) = preceded(
//        tag("-- id: "),
//        not_line_ending.map(|a: &str| SecAttr::Id(a.to_string())),
//    )(source.trim())?;
//    // dbg!(&a);
//    // dbg!(&b);
//    Ok((source, attr))
//}
//fn attr_class(source: &str) -> IResult<&str, SecAttr> {
//    //dbg!("55555555555555555555555555555555555");
//    let (a, b) = preceded(tag("-- class: "), not_line_ending)(source)?;
//    // dbg!(&a);
//    // dbg!(&b);
//    Ok((source, SecAttr::None))
//}
//fn attr_misc(source: &str) -> IResult<&str, SecAttr> {
//    let (a, b) = preceded(tag("-- x: "), not_line_ending)(source)?;
//    dbg!(&a);
//    dbg!(&b);
//    Ok((source, SecAttr::None))
//}

enum TmpAttr {
    Classes(String),
    Id(String),
    Misc(String),
}

pub fn textarea(source: &str) -> IResult<&str, Section> {
    let (source, _) = tuple((tag_no_case("-- textarea"), not_line_ending, line_ending))(source)?;
    dbg!("--------------------------------------");

    let (x, raw_attrs) = many0(alt((
        preceded(
            tag("-- id: "),
            not_line_ending.map(|x: &str| TmpAttr::Id(x.to_string())),
        ),
        preceded(
            tag("-- class: "),
            not_line_ending.map(|x: &str| TmpAttr::Classes(x.to_string())),
        ),
        // preceded(
        //     tag("-- id: "),
        //     not_line_ending.map(|x: &str| TmpAttr::Id(x.to_string())),
        // ),
    )))(source.trim())?;

    //let (x, raw_attributes) = opt(many1(alt((attr_id, attr_class, attr_misc))))(source)?;

    //let (x, raw_attributes) = opt(many1(alt((attr_id, attr_class, attr_misc))))(source)?;
    dbg!("========================================");
    // dbg!(&y);

    // get the content block
    let (source, content) = alt((take_until("\n\n--"), rest))(source)?;

    // // get the id
    // let (_, id_attr) = opt(
    //     preceded(pair(take_until("-- id: "), tag("-- id: ")), not_line_ending)
    //         .map(|x: &str| x.to_string()),
    // )(content)?;

    // // get the classes
    // let (_, classes) =
    //     opt(preceded(
    //         pair(take_until("-- class: "), tag("-- class: ")),
    //         not_line_ending,
    //     )
    //     .map(|x: &str| x.split(" ").into_iter().map(|y| y.to_string()).collect()))(content)?;

    let mut id: Option<String> = None;
    let mut classes: Option<Vec<String>> = None;

    raw_attrs.iter().for_each(|attr| match attr {
        TmpAttr::Id(v) => id = Some(v.to_string()),
        TmpAttr::Classes(v) => classes = Some(v.split(" ").map(|x| x.to_string()).collect()),
        _ => {}
    });

    Ok((
        source,
        Section::Textarea {
            attributes: None,
            classes,
            id_attr: id,
            text: None,
        },
    ))
}

