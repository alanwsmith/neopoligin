#![allow(unused_imports)]
// use neopolengine::neo_section::neo_section;
use neopolengine::neo_sections::neo_sections;
use neopolengine::neo_sections::Section;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SectionTestCase {
    expected: Vec<Section>,
    ignore: Option<bool>,
    input: String,
    solo: Option<bool>,
}

#[test]
fn solo_test_specs() {
    let json_text = fs::read_to_string("./tests/neopolitan_spec_tests.json").unwrap();
    let section_test_cases: Vec<SectionTestCase> =
        serde_json::from_str(json_text.as_str()).unwrap();

    // do solo tests first
    section_test_cases
        .iter()
        .filter(|t| match t.solo {
            Some(just_me) if just_me == true => true,
            _ => false,
        })
        .into_iter()
        .for_each(|x| {
            dbg!(&x.input);
            let results = neo_sections(&x.input).unwrap().1;
            assert_eq!(x.expected, results);
            ()
        });

    // then loop through everything that's not ignored
    // (this redoes the solo tests, but that's fine for now)
    section_test_cases
        .iter()
        .filter(|t| match t.ignore {
            Some(skip_me) if skip_me == true => false,
            _ => true,
        })
        .into_iter()
        .for_each(|x| {
            dbg!(&x.input);
            let results = neo_sections(&x.input).unwrap().1;
            assert_eq!(x.expected, results);
            ()
        });
}
