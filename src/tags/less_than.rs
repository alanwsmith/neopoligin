use crate::tags::Tag;
use nom::bytes::complete::tag;
use nom::character::complete::none_of;
use nom::sequence::pair;
use nom::IResult;

pub fn less_than(source: &str) -> IResult<&str, Tag> {
    let (source, text) = pair(tag("<"), none_of("<"))(source)?;
    Ok((
        source,
        Tag::LessThan {
            text: format!("{}{}", text.0, text.1),
        },
    ))
}

#[cfg(test)]

mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "< ", 
        Tag::LessThan{ text: "< ".to_string() })]
    fn less_than_runner(#[case] input: &str, #[case] expected: Tag) {
        assert_eq!(expected, less_than(input).unwrap().1)
    }
}
