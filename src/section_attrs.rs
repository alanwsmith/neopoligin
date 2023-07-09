use crate::section_attrs::class::class;
use crate::section_attrs::id::id;
use nom::multi::many0;
use nom::IResult;
use nom::branch::alt;
use serde::Serialize;

pub mod class;
pub mod id;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum SecAttr {
    Class(Vec<String>),
    Id(String),
}

pub fn sec_attrs(source: &str) -> IResult<&str, Vec<SecAttr>> {
    let (source, attrs) = many0(alt((class, id)))(source.trim())?;
    Ok((source, attrs))
}
