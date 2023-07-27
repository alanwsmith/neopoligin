use crate::helpers::empty_line::empty_line;
use crate::neo_sections::neo_section;
use crate::neo_sections::NeoSection;
use minijinja::value::{StructObject, Value};
use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub source_hash: Option<String>,
    pub path: Option<PathBuf>,
    pub source: String,
    pub section_storage: Option<Vec<NeoSection>>,
}

impl Page {
    pub fn new_from(source: &str) -> Page {
        Page {
            path: None,
            source_hash: None,
            source: source.to_string(),
            section_storage: None,
        }
    }
}

impl Page {
    pub fn raw_sections(&mut self) -> Vec<NeoSection> {
        match &self.section_storage {
            Some(x) => x.clone(),
            None => {
                self.section_storage = Some(page(self.source.as_str()).unwrap().1);
                self.section_storage.clone().unwrap()
            }
        }
    }
}

impl Page {
    pub fn title_data(&mut self) -> NeoSection {
        self.raw_sections().into_iter().nth(0).unwrap()
    }
}

impl StructObject for Page {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "title_data" => Some(Value::from_serializable(&self.clone().title_data())),
            _ => None,
        }
    }
}

// impl Page {
//
//     pub fn title_section(&self) -> Option<NeoSection> {
//         None
//     }
// }

// impl Page {
//     // pub fn title(&self) -> Option<NeoSection> {
//     //     // "asdfasdfasdf".to_string()
//     //     Some(self.parse_title().unwrap().1)
//     // }
//     // pub fn parse_title(&self) -> IResult<&str, NeoSection, VerboseError<&str>> {
//     //     Ok(title_section(self.raw_file.as_str())?)
//     //     // "BOLERO".to_string()
//     // }
// }

// impl Page {
//     pub fn new_from(source: &str) -> Page {
//         let mut p = Page {
//             // attributes: None,
//             blurb: None,
//             categories: None,
//             config: None,
//             css: None,
//             date: None,
//             head: None,
//             id: None,
//             // references: None,
//             path: None,
//             sections: None,
//             scripts: None,
//             source_hash: None,
//             status: None,
//             template: None,
//             time: None,
//             // title: None,
//             r#type: None,
//             universe: None,
//         };
//         let raw_sections = page(source).unwrap().1;
//         let filtered_sections: Option<Vec<NeoSection>> = Some(
//             raw_sections
//                 .into_iter()
//                 .filter_map(|sec| match sec {
//                     NeoSection::RawPageAttributes(key_values) => {
//                         key_values.iter().for_each(|(key, value)| {
//                             match key.to_lowercase().trim() {
//                                 "date" => {
//                                     p.date = Some(value.trim().to_string());
//                                 }
//                                 "id" => {
//                                     p.id = Some(value.trim().to_string());
//                                 }
//                                 "status" => {
//                                     p.status = Some(value.trim().to_string());
//                                 }
//                                 "time" => {
//                                     p.time = Some(value.trim().to_string());
//                                 }
//                                 "type" => {
//                                     p.r#type = Some(value.trim().to_string());
//                                 }
//                                 _ => {}
//                             }
//                             ()
//                         });
//                         None
//                     }
//                     x => Some(x),
//                 })
//                 .collect(),
//         );
//         if filtered_sections.as_ref().unwrap().len() == 0 {
//             p.sections = None;
//         } else {
//             p.sections = filtered_sections;
//         }
//         p
//     }
// }

pub fn page(source: &str) -> IResult<&str, Vec<NeoSection>, VerboseError<&str>> {
    let (source, sections) =
        separated_list1(empty_line, preceded(multispace0, neo_section))(source)?;
    Ok((source, sections))
}

// impl Page {
//     pub fn title(&self) -> String {
//         "I AM SPARTICS".to_string()
//     }
//     pub fn tango(&self) -> String {
//         // "TANGO".to_string()
//         self.universe.clone().expect("BOOM?").rumba()
//     }
// }
