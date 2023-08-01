



use crate::neo_sections::NeoSection;


use nom::bytes::complete::tag;

use nom::character::complete::line_ending;


use nom::character::complete::not_line_ending;

use nom::character::complete::space0;




use nom::error::VerboseError;




use nom::sequence::pair;
use nom::sequence::preceded;

use nom::IResult;



use std::path::PathBuf;
use walkdir::WalkDir;

pub fn image_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- image")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, basename) = preceded(tag("-- "), not_line_ending)(source)?;
    // dbg!(&source);
    // let (source, _) = line_ending(source)?;
    // let (source, attributes) = attributes(source)?;
    // let (source, _) = empty_line(source)?;
    // let (source, headline) = opt(block)(source)?;
    // let (source, content) = opt(many1(block))(source)?;
    Ok((
        source,
        NeoSection::Image {
            attributes: None,
            caption: None,
            src: get_file_path(basename),
        },
    ))
}

fn get_file_path(target_name: &str) -> Option<String> {
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
                    Some(return_path_buf.display().to_string())
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
