use crate::tag_attrs::class::class;
use nom::branch::alt;
use nom::multi::many0;
use nom::IResult;
use crate::tag_attrs::id::id;
use serde::Serialize;

pub mod class;
pub mod id;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum TagAttr {
    Class(Vec<String>),
    Id(String),
}

pub fn tag_attrs(source: &str) -> IResult<&str, Vec<TagAttr>> {
    let (source, attrs) = many0(alt((class, id)))(source)?;
    Ok((source, attrs))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::Error;
    use nom::Err;
    use rstest::rstest;

    #[rstest]
    #[case("|class: delta echo>>", Ok((">>", vec![TagAttr::Class(vec!["delta".to_string(), "echo".to_string()])])))]
    fn tag_attrs_tester(
        #[case] i: &str,
        #[case] e: Result<(&str, Vec<TagAttr>), Err<Error<&str>>>,
    ) {
        assert_eq!(e, tag_attrs(i));
    }
}
