#![allow(unused_imports)]
use neopolengine::sections::Section;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct TestFrame {
//     section_tests: Vec<SectionTestCase>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct SectionTestCase {
//     input: String,
//     expected: Section,
// }

// #[test]
// fn xsolo_test_specs() {
//     let json_text = fs::read_to_string("./tests/neopolitan_spec_tests.json").unwrap();
//     let tf: TestFrame = serde_json::from_str(json_text.as_str()).unwrap();
//     dbg!(tf);
//     assert_eq!(2, 2);
// }

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
