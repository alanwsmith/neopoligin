use nom::IResult;

pub fn list_item(source: &str) -> IResult<&str, &str> {
    Ok((source, ""))
}
