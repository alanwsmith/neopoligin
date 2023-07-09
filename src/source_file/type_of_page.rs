use crate::source_file::SourceFile;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::not_line_ending;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

impl SourceFile {
    pub fn type_of_page<'a>(
        &'a self,
        source: &'a str,
    ) -> IResult<&'a str, &'a str> {
        let (source, type_of_page) = preceded(
            tuple((
                take_until("-> attributes"),
                take_until(">> type: "),
                tag(">> type: "),
            )),
            not_line_ending,
        )(source)?;
        Ok((source, type_of_page))
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::SourceFile;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-> attributes", ">> type: alfa"].join("\n"),
        "alfa"
        )]
    fn type_tester(#[case] i1: String, #[case] e: &str) {
        let sf = SourceFile::new();
        assert_eq!(e, sf.type_of_page(i1.as_str()).unwrap().1);
    }
}
