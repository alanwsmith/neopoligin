use crate::tag_attrs::TagAttr;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub fn class(source: &str) -> IResult<&str, TagAttr> {
    let (source, value_string) =
        preceded(tag("|class: "), is_not("|>"))(source)?;
    let (_, classes) = separated_list1(
        space1,
        is_not(" ").map(|s: &str| s.to_string()),
    )(value_string)?;
    Ok((source, TagAttr::Class(classes)))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::Error;
    use nom::Err;
    use rstest::rstest;

    #[rstest]
    #[case("|class: alfa bravo>>", Ok((">>", TagAttr::Class(vec!["alfa".to_string(), "bravo".to_string()]))))]
    fn class_tester(
        #[case] i: &str,
        #[case] e: Result<(&str, TagAttr), Err<Error<&str>>>,
    ) {
        assert_eq!(e, class(i));
    }
}
