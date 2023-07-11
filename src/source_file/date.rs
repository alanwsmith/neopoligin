use crate::source_file::SourceFile;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::not_line_ending;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

impl SourceFile {
    pub fn date<'a>(
        &self,
        source: &'a str,
        format: &'a str,
    ) -> IResult<&'a str, Option<String>> {
        let (source, date) = preceded(
            tuple((
                take_until("-- attributes"),
                take_until("-- date: "),
                tag("-- date: "),
            )),
            not_line_ending,
        )(source)?;
        let dt: DateTime<Utc> =
            Utc.datetime_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
        Ok((source, Some(dt.format(format).to_string())))
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::SourceFile;
    use rstest::rstest;

    #[rstest]
    #[case(
        vec!["-- attributes", "-- date: 2023-07-06 12:24:00"].join("\n"),
        "%B %Y",
        "July 2023".to_string()
        )]
    fn date_tester(#[case] i1: String, #[case] i2: &str, #[case] e: String) {
        let sf = SourceFile::new();
        assert_eq!(e, sf.date(i1.as_str(), i2).unwrap().1.unwrap());
    }
}
