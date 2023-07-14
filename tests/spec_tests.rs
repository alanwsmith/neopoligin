#![allow(unused_imports)]
use neopolengine::sections::section::section;
use neopolengine::sections::Section;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SectionTestCase {
    input: String,
    expected: Section,
}

#[test]
fn xsolo_test_specs() {
    let json_text = fs::read_to_string("./tests/neopolitan_spec_tests.json").unwrap();
    let section_test_cases: Vec<SectionTestCase> =
        serde_json::from_str(json_text.as_str()).unwrap();
    section_test_cases.iter().for_each(|t| {
        let results = section(&t.input).unwrap().1;
        dbg!(&t.input);
        assert_eq!(t.expected, results);
        ()
    });
}

//     "expected": {
//       "sections": [
//         {
//           "type": "Section",
//           "kind": "Title",
//           "attrs": [],
//           "headline": {
//             "type": "Block",
//             "kind": "Headline",
//             "snippets": [
//               {
//                 "type": "Snippet",
//                 "kind": "Text",
//                 "string": "Alfa Bravo"
//               }
//             ]
//           }
//         }
//       ]
//     }
//   }
// ]
