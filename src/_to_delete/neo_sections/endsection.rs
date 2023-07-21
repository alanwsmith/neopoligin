use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::sequence::tuple;
use nom::IResult;

pub fn endsection(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- endsection"),
        not_line_ending,
        newline,
    ))(source)?;
    Ok((
        source,
        NeoSection::EndSection
    ))
}
