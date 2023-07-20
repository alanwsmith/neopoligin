use crate::attrs::attrs;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::multi::many1;
use nom::sequence::tuple;
use crate::containers::checklistitem::checklistitem;
use nom::IResult;

pub fn checklist(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- checklist"),
        not_line_ending,
        newline,
    ))(source)?;
    let (source, attrs) = attrs(source)?;
    //let (source, paragraphs) = many_till(paragraph, eof)(source)?;

    let (source, items) = many1(checklistitem)(source)?;
    Ok((
        source,
        NeoSection::Checklist {
            attrs,
            blocks: None,
            items: Some(items),
        },
    ))
}
