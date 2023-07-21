use crate::attrs::Attribute;
use crate::attrs::AutoCapitalizeValue;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::IResult;
use nom::sequence::pair;
use nom::bytes::complete::tag_no_case;
use nom::branch::alt;
use nom::Parser;


pub fn autocapitalize(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = tag("-- autocapitalize: ")(source)?;
    let (source, attr) = alt((
        tag_no_case("off").map(|_| AutoCapitalizeValue::Off),
        tag_no_case("none").map(|_| AutoCapitalizeValue::None),
        tag_no_case("sentences").map(|_| AutoCapitalizeValue::Sentences),
        tag_no_case("words").map(|_| AutoCapitalizeValue::Words),
        tag_no_case("characters").map(|_| AutoCapitalizeValue::Characters),
        tag_no_case("on").map(|_| AutoCapitalizeValue::On)
    ))(source)?;
    let(source, _) = pair(not_line_ending, line_ending)(source)?;
    Ok((source, Attribute::AutoCapitalize(attr)))
}
