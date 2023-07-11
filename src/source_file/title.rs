use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, String> {
    let (source, _) = take_until("-- title")(source)?;
    let (source, title) = preceded(
        tuple((tag("-- title"), multispace1)),
        not_line_ending,
    )(source)?;

    Ok((source, title.trim().to_string()))
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    pub fn basic_test() {
        let lines = vec!["-- title", "", "Delta Echo"].join("\n");
        let expected = "Delta Echo".to_string();
        assert_eq!(expected, title(lines.as_str()).unwrap().1);
    }

    //
}
