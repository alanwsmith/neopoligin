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
    ignore: Option<bool>,
    input: String,
    expected: Vec<Section>,
}

#[test]
fn solo_test_specs() {
    let json_text = fs::read_to_string("./tests/neopolitan_spec_tests.json").unwrap();
    let section_test_cases: Vec<SectionTestCase> =
        serde_json::from_str(json_text.as_str()).unwrap();
    section_test_cases.iter().for_each(|t| {
        dbg!(&t.input);
        // not sure this is working yet. proceded with caution
        match t.ignore {
            Some(skip) if skip == true => {}
            _ => {
                let results = neo_sections(&t.input).unwrap().1;
                assert_eq!(t.expected, results);
            }
        }
        ()
    });
}
