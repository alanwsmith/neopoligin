use crate::section_attrs::sec_attrs;
use crate::sections::alt;
use crate::neo_sections::Section;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn image(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-- image"), not_line_ending, line_ending))(source.trim())?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source)?;
    let (content, id) = opt(preceded(tag("-- "), not_line_ending))(content)?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (_, alt) = alt((take_until("\n\n--"), rest))(content)?;
    let n1 = get_path_to_file(&id.unwrap());
    // dbg!(&n1);
    if let Some(n2) = n1 {
        // let n2 = n1.unwrap();
        let n3 = n2.to_str();
        if let Some(n4) = n3 {
            // let n4 = n3.unwrap();
            let n5 = n4.to_string();
            // let new_source2 = new_source.to_str().unwrap();
            Ok((
                source,
                Section::Image {
                    alt: alt.to_string(),
                    attrs,
                    src: n5,
                },
            ))
        } else {
            Ok((
                source,
                Section::Image {
                    alt: alt.to_string(),
                    attrs,
                    src: "".to_string(),
                },
            ))
        }
    } else {
        Ok((
            source,
            Section::Image {
                alt: alt.to_string(),
                attrs,
                src: "".to_string(),
            },
        ))
    }
}

fn get_path_to_file(target_name: &str) -> Option<PathBuf> {
    let site_root = "/Users/alan/workshop/alanwsmith.com/content";
    let files: Vec<_> = WalkDir::new(site_root)
        .into_iter()
        .filter_map(|v| {
            if let Some(name) = v.as_ref().unwrap().path().file_stem() {
                let target_name_stem = PathBuf::from(target_name);
                let asdf = target_name_stem.file_stem().unwrap();
                if name.to_str() == asdf.to_str() {
                    let dir = v.as_ref().unwrap().path().strip_prefix(site_root);
                    let mut return_path_buf = PathBuf::from("/");
                    return_path_buf.push(dir.unwrap().to_str().unwrap());
                    //dbg!(&return_path_buf);
                    Some(return_path_buf)
                    // v.ok()
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if files.len() == 1 {
        Some(files[0].clone())
        // let new_path = PathBuf::from(files[0].unwrap());
        // let new_path = files[0].to_path_buf().strip_prefix(site_root).unwrap();
        // dbg!(&new_path);
        // let new_path2 = &new_path.to_str().unwrap();
        // Some("".to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod text {
    use super::*;
    // use crate::blocks::Block;
    // use crate::section_attrs::SecAttr;
    use crate::neo_sections::Section;
    // use crate::tags::Tag;
    use rstest::rstest;

    #[rstest]
    // WARNING: This is a hard coded test on my file system.
    // I'm ignoring it for now so it doesnt' cause issues, but
    // if you run it, a file needs to exist at the right path
    #[ignore]
    #[case(
        vec!["-- image", "-- diff2html-side-by-side","", "Charlie Delta", "", "-- next"].join("\n"), 
        Ok(("\n\n-- next", 
        Section::Image {
            alt: "Charlie Delta".to_string(),
            attrs: vec![],
            src: "/images/2023/diff2html-side-by-side.png".to_string(),
        }))
    )]

    // #[case(
    //     vec!["-> youtube", ">> deltaecho",">> class: foxtrot", "", "whiskey tango", "", "-> next"].join("\n"),
    //     Ok(("\n\n-> next",
    //     Section::Image {
    //         attrs: vec![
    //             SecAttr::Class(vec!["foxtrot".to_string()])
    //         ],
    //         id: "deltaecho".to_string(),
    //         paragraphs: vec![Block::Paragraph {
    //             tags: vec![Tag::Text {
    //                 text: "whiskey tango".to_string(),
    //             }],
    //         }],
    //     }
    //         ))
    // )]

    fn image_test(#[case] i: String, #[case] e: IResult<&str, Section>) {
        assert_eq!(e, image(i.as_str()))
    }
}
