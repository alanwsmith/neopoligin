use crate::attributes::attributes;
use crate::attributes::AttributesObj;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::sequence::terminated;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Snippet {
    Link {
        attributes: AttributesObj,
        url: String,
        string: String,
    },
    Strong {
        attributes: AttributesObj,
        string: String,
    },
    Text {
        string: String,
    },
    None,
}

pub fn contents(source: &str) -> IResult<&str, Snippet> {
    let (source, snippet) = alt((link, strong, text))(source)?;
    Ok((source, snippet))
}

pub fn link(source: &str) -> IResult<&str, Snippet> {
    let (source, _) = tag_no_case("<<link|")(source)?;
    let (source, string) = terminated(is_not("|"), tag("|"))(source)?;
    let (source, url) = is_not("|>")(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Snippet::Link {
            url: url.to_string(),
            string: string.to_string(),
            attributes,
        },
    ))
}

pub fn strong(source: &str) -> IResult<&str, Snippet> {
    let (source, _) = tag_no_case("<<strong|")(source)?;
    let (source, content) = is_not("|>")(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Snippet::Strong {
            string: content.to_string(),
            attributes,
        },
    ))
}

pub fn text(source: &str) -> IResult<&str, Snippet> {

    let (source, content) = is_not("\n<")(source)?;
    Ok((
        source,
        Snippet::Text {
            string: content.to_string(),
        },
    ))
    
}
