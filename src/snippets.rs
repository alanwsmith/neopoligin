use crate::attributes::attributes;
use crate::attributes::Attribute;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::IResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Snippet {
    Text {
        string: String,
    },
    Strong {
        string: String,
        attributes: Option<Vec<Attribute>>,
    },
    None,
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

