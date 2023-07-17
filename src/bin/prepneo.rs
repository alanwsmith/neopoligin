#![allow(unused_imports)]
use core::fmt::Error;
use miette::{IntoDiagnostic, Result};
use neopolengine::build_site::build_site;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::fs;
use std::fs::copy;
use std::path::PathBuf;
use std::time::Duration;
use watchexec::{
    action::{Action, Outcome},
    config::{InitConfig, RuntimeConfig},
    handler::PrintDebug,
    Watchexec,
};
use watchexec_signals::Signal;

// This is currently hard coded to look for:
// `>> site: neoengine``
//
// Note this is current etup to look at .org files
// until the move to raw .neo files in the grimoire is made
//
// [] Only allows specific nonce- words
// [] Move nonce word setup into config
// [] Add a note to the top of the files saying that they are copies
// and should not be edited. Point to full path
// [] Move config to json or yaml file
//  to help prevenet editing the wrong ones
// [] Handle dir paths if there is no id
// [x] Handle explicit paths
// [x] Configure site directory and subdirectory paths independently
// [x] Only allows .neo extensions
// [x] Only allows published and draft files
// [x] Make direcotires if necessary
// [x] Ouput goes into a sub directory if one is defined
// [x] Each file has its own directory created for it
// [x] The output file is always index.neo in the folder
// [x] Strip `.` from url

#[derive(Clone)]
struct Config {
    output_root: String,
    output_sub_dir: Option<String>,
    input_root: String,
    site_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));
    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["/Users/alan/Grimoire"]);
    runtime.action_throttle(Duration::new(0, 100000000));
    let we = Watchexec::new(init, runtime.clone())?;
    runtime.on_action(move |action: Action| async move {
        let mut stop_running = false;
        let mut load_content = false;
        for event in action.events.iter() {
            event.signals().for_each(|sig| match sig {
                Signal::Interrupt => {
                    stop_running = true;
                }
                _ => {}
            });
            if event
                .paths()
                .any(|(p, _)| p.starts_with("/Users/alan/Grimoire"))
            {
                load_content = true;
            }
        }
        if stop_running {
            action.outcome(Outcome::Exit);
        }
        if load_content {
            load_content_from_grimoire();
        }
        Ok::<(), Error>(())
    });
    let _ = we.reconfigure(runtime);
    let _ = we.main().await.into_diagnostic()?;
    Ok(())
}

fn load_content_from_grimoire() {
    println!("Running neoprep");

    // let neopolengine_dev = Config {
    //     input_root: "/Users/alan/workshop/neopolengine/content_raw_samples".to_string(),
    //     output_root: "/Users/alan/workshop/neopolengine/content".to_string(),
    //     output_sub_dir: Some("pages".to_string()),
    //     site_id: "neopolengine".to_string(),
    // };

    // let dev = Config {
    //     input_root: "/Users/alan/workshop/neopolengine/samples/input".to_string(),
    //     output_root: "/Users/alan/workshop/neopolengine/samples/output".to_string(),
    //     output_sub_dir: Some("pages".to_string()),
    //     site_id: "aws".to_string(),
    // };

    // REMEMBER TO CHANGE FILE EXTENSION BELOW ONCE YOU'RE
    // USING .neo FILES
    let prod = Config {
        input_root: "/Users/alan/Grimoire".to_string(),
        output_root: "/Users/alan/workshop/alanwsmith.com/content".to_string(),
        output_sub_dir: Some("pages".to_string()),
        site_id: "aws".to_string(),
    };

    let config = prod.clone();
    let paths = filter_extensions(
        fs::read_dir(&config.input_root)
            .unwrap()
            .into_iter()
            .map(|p| p.expect("here").path())
            .collect::<Vec<PathBuf>>(),
    );

    paths.iter().for_each(|p| {
        let data = fs::read_to_string(p).unwrap();
        // dbg!(&p);
        match (
            filter_status(data.as_str()).unwrap().1,
            filter_site(data.as_str(), config.site_id.as_str())
                .unwrap()
                .1,
            valid_nonce(p.to_path_buf()),
        ) {
            (true, true, true) => {
                let the_file_id = file_id(data.as_str()).unwrap().1.to_string();
                let output_dir_path = PathBuf::from(&config.output_root);
                let mut output_file_path = output_dir_path.clone();
                match override_path(data.as_str()).unwrap().1 {
                    Some(path) => {
                        output_file_path.push(path);
                    }
                    None => {
                        match &config.output_sub_dir {
                            Some(dir) => {
                                output_file_path.push(dir);
                            }
                            None => {}
                        }
                        output_file_path.push(
                            output_dir_name(
                                p.file_stem().unwrap().to_str().unwrap(),
                                the_file_id.as_str(),
                            )
                            .unwrap()
                            .1,
                        );
                        output_file_path.push("index.neo");
                    }
                }
                match output_file_path.parent() {
                    Some(path) => match path.try_exists() {
                        Ok(check) => {
                            // dbg!(&path);
                            if check == false {
                                fs::create_dir_all(path).unwrap();
                            }
                        }
                        Err(_) => {}
                    },
                    None => {}
                }
                // println!("Copying to: {}", &output_file_path.display());
                let _ = copy(p, output_file_path);
            }
            _ => { // dbg!("skipping");
            }
        }
        ()
    });

    println!("Process complete");
}

pub fn file_id(source: &str) -> IResult<&str, &str> {
    let (a, _b) = take_until("\n-- attributes")(source)?;
    let (a, _b) = tag("\n-- attributes")(a)?;
    let (a, _b) = take_until("-- id: ")(a)?;
    let (a, _b) = tag("-- id: ")(a)?;
    let (a, _b) = multispace0(a)?;
    let (_a, b) = not_line_ending(a)?;
    Ok(("", b.trim()))
}

pub fn output_dir_name<'a>(source: &'a str, id: &'a str) -> IResult<&'a str, String> {
    let (source, _) = multispace0(source.trim())?;
    let (source, parts) =
        many_till(terminated(is_not(" -.'"), alt((is_a(" -.'"), eof))), eof)(source)?;
    let response = format!(
        "{}--{}",
        parts
            .0
            .iter()
            .map(|p| p.to_lowercase())
            .collect::<Vec<String>>()
            .join("-"),
        id
    );
    Ok((source, response))
}

pub fn filter_status(source: &str) -> IResult<&str, bool> {
    let (source, check_status_1) = opt(take_until("\n-- attributes"))(source)?;
    match check_status_1 {
        Some(_) => {
            let (source, _) = tag("\n-- attributes")(source)?;
            let (source, check_status_2) = opt(take_until("-- status: "))(source)?;
            match check_status_2 {
                Some(_) => {
                    let (source, _) = tag("-- status: ")(source)?;
                    let (source, b) = not_line_ending(source)?;
                    match b.trim() {
                        "published" => Ok((source, true)),
                        "draft" => Ok((source, true)),
                        "scratch" => Ok((source, true)),
                        _ => Ok((source, false)),
                    }
                }
                None => Ok((source, false)),
            }
        }
        None => Ok((source, false)),
    }
}

pub fn filter_site<'a>(source: &'a str, site_id: &'a str) -> IResult<&'a str, bool> {
    let (source, check_site_1) = opt(take_until("\n-- attributes"))(source)?;
    match check_site_1 {
        Some(_) => {
            let (source, _) = tag("\n-- attributes")(source)?;
            let (source, check_site_2) = opt(take_until("-- site: "))(source)?;
            match check_site_2 {
                Some(_) => {
                    let (source, _) = tag("-- site: ")(source)?;
                    let (source, the_id) = not_line_ending(source)?;
                    if the_id.trim() == site_id {
                        Ok((source, true))
                    } else {
                        Ok((source, false))
                    }
                }
                None => Ok((source, false)),
            }
        }
        None => Ok((source, false)),
    }
}

pub fn filter_extensions(list: Vec<PathBuf>) -> Vec<PathBuf> {
    list.into_iter()
        .filter(|p| match p.extension() {
            Some(ext) => {
                if ext == "org" {
                    true
                } else {
                    false
                }
            }
            None => false,
        })
        .collect()
}

pub fn override_path(source: &str) -> IResult<&str, Option<String>> {
    let (source, _) = pair(take_until("\n-- attributes"), tag("\n-- attributes"))(source)?;
    let (source, the_path) = opt(preceded(
        pair(take_until("-- path: "), tag("-- path: ")),
        not_line_ending.map(|s: &str| s.to_string()),
    ))(source)?;
    Ok((source, the_path))
}

pub fn valid_nonce(p: PathBuf) -> bool {
    // TODO: Remove `aws-` to see if there's anytying there that needs
    // to be removed
    // Don't do `cloudinary- ` until you've scrubbed it
    let nonces = vec![
        "alt- ",
        "ansible- ",
        "apis- ",
        "app- ",
        "applescript- ",
        "apps- ",
        "ascii- ",
        "audition- ",
        "automator- ",
        "awk- ",
        "bash- ",
        "bbedit- ",
        "blender- ",
        "bookmarks- ",
        "books- ",
        "chrome- ",
        "classnotes- ",
        "cli- ",
        "colors- ",
        "confnotes- ",
        "css- ",
        "cuc- ",
        "d3- ",
        "daily-links- ",
        "data- ",
        "davinci- ",
        "design- ",
        "dev- ",
        "django- ",
        "docker- ",
        "drupal- ",
        "eclipse- ",
        "emacs- ",
        "electron- ",
        "examples- ",
        "exiftool- ",
        "ffmpeg- ",
        "freenas- ",
        "gatsby- ",
        "gif- ",
        "github- ",
        "grim- ",
        "grub- ",
        "hammerspoon-",
        "heroku- ",
        "html- ",
        "htpc- ",
        "httrack- ",
        "hugo- ",
        "iterm2- ",
        "jekyll- ",
        "jq- ",
        "jquery- ",
        "js- ",
        "json- ",
        "keyboard-maestro- ",
        "keyboards- ",
        "kindle- ",
        "launchd- ",
        "ligthroom- ",
        "lists- ",
        "lua- ",
        "image-magick- ",
        "minecraft- ",
        "misc- ",
        "music- ",
        "musicbrainz- ",
        "neo- ",
        "neoe- ",
        "neop- ",
        "netlify- ",
        "nextjs- ",
        //

        //
        "post- ",
        "site- ",
        "stream- ",
        "tools- ",
    ];

    match nonces.iter().find(|&&n| {
        p.file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap()
            .starts_with(n)
    }) {
        Some(_) => true,
        None => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    pub fn filter_extensions_test() {
        let files = vec![
            PathBuf::from("/a/b/alfa.org"),
            PathBuf::from("/a/b/bravo.txt"),
        ];
        assert_eq!(
            vec![PathBuf::from("/a/b/alfa.org")],
            filter_extensions(files)
        );
    }

    #[test]
    pub fn filter_status_false() {
        // allowed statuses are hard coded above
        let lines = [
            "",
            "-- attributes",
            "-- id: 12341234",
            "-- status: unpublished",
        ];
        assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, false);
    }

    #[test]
    pub fn filter_status_true() {
        let lines = [
            "",
            "-- attributes",
            "-- id: 12341234",
            "-- status: published",
        ];
        assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, true);
    }

    #[test]
    pub fn filter_status_with_trailing_space() {
        let lines = [
            "",
            "-- attributes",
            "-- id: 12341234",
            "-- status: published ",
        ];
        assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, true);
    }

    #[test]
    pub fn filter_status_with_no_content() {
        let lines = ["", "-- attributes", "-- date: 2023-02-03 13:14:15"];
        assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, false);
    }

    #[test]
    pub fn filter_site_test() {
        let lines = [
            "",
            "-- attributes",
            "-- id: 12341234",
            "-- site: neoengine",
            "-- status: published ",
        ];
        assert_eq!(
            filter_site(lines.join("\n").as_str(), "neoengine")
                .unwrap()
                .1,
            true
        );
    }

    #[test]
    pub fn filter_site_test_with_no_attibutes() {
        let lines = ["this is a file with no attributes"];
        assert_eq!(
            filter_site(lines.join("\n").as_str(), "neoengine")
                .unwrap()
                .1,
            false
        );
    }

    #[test]
    pub fn basic_output_dir_name() {
        let source = PathBuf::from("/some/posts/rust- Basic Path Example.neo");
        let id = String::from("1234qwer");
        let expected = Ok(("", "rust-basic-path-example--1234qwer".to_string()));
        let results = output_dir_name(source.file_stem().unwrap().to_str().unwrap(), id.as_str());
        assert_eq!(results, expected);
    }

    #[test]
    pub fn dir_with_dashes_that_are_not_followed_by_a_space() {
        let source = PathBuf::from("alfa-bravo");
        let id = String::from("9876rewq");
        let expected = Ok(("", "alfa-bravo--9876rewq".to_string()));
        let results = output_dir_name(source.file_stem().unwrap().to_str().unwrap(), id.as_str());
        assert_eq!(results, expected);
    }

    #[test]
    pub fn file_id_basic() {
        let lines = ["", "-- attributes", "-- id: 1234alfa"].join("\n");
        assert_eq!(file_id(lines.as_str()).unwrap().1, "1234alfa");
    }

    #[test]
    pub fn file_id_with_trailing_white_space() {
        let lines = [
            "",
            "-- attributes",
            "-- id: 6789bravo ",
            "-- status: published",
            "",
        ]
        .join("\n");
        assert_eq!(file_id(lines.as_str()).unwrap().1, "6789bravo");
    }

    #[test]
    pub fn get_override_path() {
        let lines = ["", "-- attributes", "-- path: index.neo", ""].join("\n");
        assert_eq!(
            override_path(lines.as_str()).unwrap().1,
            Some("index.neo".to_string())
        );
    }

    #[test]
    pub fn valid_noce_test() {
        let name = PathBuf::from("d3- alfa bravo");
        assert_eq!(true, valid_nonce(name));
    }

    #[test]
    pub fn valid_noce_test_skip() {
        let name = PathBuf::from("skipthis- charlie delta");
        assert_eq!(false, valid_nonce(name));
    }
}
