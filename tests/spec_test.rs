#![allow(unused_imports)]
use neopolengine::neo_sections::NeoSection;
use neopolengine::page::Page;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestParts {
    input: String,
    expected: Vec<NeoSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestShell {
    tests: Vec<TestParts>,
}

#[test]
fn solo_test_specs() {
    let paths = fs::read_dir("./tests/spec").unwrap();
    for path in paths {
        let file_path = path.unwrap().path().display().to_string();
        if file_path.find(".json") != None {
            let json_text = fs::read_to_string(file_path).unwrap();
            let test_shell: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
            test_shell.tests.into_iter().for_each(|test| {
                let mut p = Page::new_from(&test.input);
                assert_eq!(test.expected, p.raw_sections());
            });
        }
    }

    // let json_text = fs::read_to_string("./spec.json").unwrap();
    // let test_data: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
    // // This does all the sections via pages
    // test_data.smoke_tests.iter().into_iter().for_each(|x| {
    //     let mut p = Page::new_from(&x.parts.input);
    //     assert_eq!(x.parts.expected, p.raw_sections());
    //     ()
    // });

    // // This does all the attributes
    // test_data.attributes.iter().into_iter().for_each(|x| {
    //     dbg!(&x.parts.input);
    //     let results = sections(&x.parts.input).unwrap().1;
    //     assert_eq!(x.parts.expected, results);
    //     ()
    // });

    // // This does all the inline_tags
    // test_data.inline_tags.iter().into_iter().for_each(|x| {
    //     dbg!(&x.parts.input);
    //     let results = sections(&x.parts.input).unwrap().1;
    //     assert_eq!(x.parts.expected, results);
    //     ()
    // });

    // // This runs any soloed tests
    // test_data
    //     .sections
    //     .iter()
    //     .filter(|t| match t.solo {
    //         Some(just_me) if just_me == true => true,
    //         _ => false,
    //     })
    //     .into_iter()
    //     .for_each(|x| {
    //         dbg!(&x.parts.input);
    //         let results = sections(&x.parts.input).unwrap().1;
    //         assert_eq!(x.parts.expected, results);
    //         ()
    //     });

    // // This skips anything that's not ignored
    // test_data
    //     .sections
    //     .iter()
    //     .filter(|t| match t.ignore {
    //         Some(skip_me) if skip_me == true => false,
    //         _ => true,
    //     })
    //     .into_iter()
    //     .for_each(|x| {
    //         dbg!(&x.parts.input);
    //         let results = sections(&x.parts.input).unwrap().1;
    //         assert_eq!(x.parts.expected, results);
    //         ()
    //     });

    // // These are the old tests that should be
    // // removed once the sections_v2 stuff is onlin
    // let section_test_cases: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
    // // do solo tests first
    // section_test_cases
    //     .spec_tests
    //     .iter()
    //     .filter(|t| match t.solo {
    //         Some(just_me) if just_me == true => true,
    //         _ => false,
    //     })
    //     .into_iter()
    //     .for_each(|x| {
    //         dbg!(&x.parts.input);
    //         let results = neo_sections(&x.parts.input).unwrap().1;
    //         assert_eq!(x.parts.expected, results);
    //         ()
    //     });

    // // then loop through everything that's not ignored
    // // (this redoes the solo tests, but that's fine for now)
    // section_test_cases
    //     .spec_tests
    //     .iter()
    //     .filter(|t| match t.ignore {
    //         Some(skip_me) if skip_me == true => false,
    //         _ => true,
    //     })
    //     .into_iter()
    //     .for_each(|x| {
    //         // dbg!(&x.parts.input);
    //         let results = neo_sections(&x.parts.input).unwrap().1;
    //         assert_eq!(x.parts.expected, results);
    //         ()
    //     });

    //
}
