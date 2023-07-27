use crate::source_file::SourceFile;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

impl SourceFile {
    pub fn template<'a>(
        &'a self,
        source: &'a str,
    ) -> IResult<&'a str, &'a str> {
        let (source, _) = take_until("-- attributes")(source)?;
        let (source, template) = delimited(
            tuple((take_until("-- type: "), tag_no_case("-- type: "))),
            not_line_ending,
            alt((line_ending, eof)),
        )(source)?;
        let return_value = template.trim();
        Ok((source, return_value))
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::SourceFile;

    #[test]
    pub fn basic_test() {
        let sf = SourceFile::new();
        let lines = vec!["-- attributes", "-- type: alfa"].join("\n");
        let expected = "alfa";
        assert_eq!(expected, sf.template(lines.as_str()).unwrap().1);
    }

    #[test]
    pub fn basic_with_whitespace_problem() {
        let sf = SourceFile::new();
        let lines = vec!["-- attributes", "-- type: bravo "].join("\n");
        let expected = "bravo";
        assert_eq!(expected, sf.template(lines.as_str()).unwrap().1);
    }
}
