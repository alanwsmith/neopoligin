use crate::blocks::Block;
use crate::sections::sections;
use crate::sections::Section;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub attributes: Option<HashMap<String, String>>,
    pub blurb: Option<Vec<Block>>,
    pub categories: Option<Vec<String>>,
    pub config: Option<HashMap<String, String>>,
    pub css: Option<Vec<String>>,
    pub date: Option<String>,
    pub head: Option<Vec<String>>,
    pub id: Option<String>,
    pub path: Option<PathBuf>,
    pub references: Option<Vec<Reference>>,
    pub scripts: Option<Vec<String>>,
    pub sections: Option<Vec<Section>>,
    pub source: String,
    pub status: Option<String>,
    pub template: Option<String>,
    pub time: Option<String>,
    pub title: Option<Vec<Block>>,
    pub r#type: Option<String>,
}

impl Page {
    pub fn run_parser(&mut self) {
        // dbg!("running parser");
        let raw_sections = sections(&self.source).unwrap().1;
        // dbg!(&raw_sections);

        let filtered_sections: Option<Vec<Section>> = Some(
            raw_sections
                .into_iter()
                .filter_map(|sec| match sec {
                    Section::RawPageAttributes(key_values) => {
                        print!("EEEEEEEEEEEEEEE");
                        key_values.iter().for_each(|(key, value)| {
                            match key.to_lowercase().trim() {
                                "date" => {
                                    self.date = Some(value.trim().to_string());
                                }
                                "id" => {
                                    self.id = Some(value.trim().to_string());
                                }
                                "status" => {
                                    self.status = Some(value.trim().to_string());
                                }
                                "time" => {
                                    self.time = Some(value.trim().to_string());
                                }
                                "type" => {
                                    self.r#type = Some(value.trim().to_string());
                                }
                                _ => {}
                            }
                            ()
                        });
                        None
                    }
                    x => {
                        print!(".");
                        Some(x)
                    }
                })
                .collect(),
        );
        if filtered_sections.as_ref().unwrap().len() == 0 {
            self.sections = None;
        } else {
            self.sections = filtered_sections;
        }

        //
    }
}

impl Page {
    pub fn new_from(source: &str) -> Page {
        Page {
            attributes: None,
            blurb: None,
            categories: None,
            config: None,
            css: None,
            date: None,
            head: None,
            id: None,
            references: None,
            path: None,
            sections: None,
            scripts: None,
            source: source.to_string(),
            status: None,
            template: None,
            time: None,
            title: None,
            r#type: None,
        }

        // let raw_sections = sections(source).unwrap().1;

        // &raw_sections.into_iter().for_each(|sec| match sec {
        //     Section::RawPageAttributes(key_values) => {
        //         dbg!("--------------------");
        //         key_values.iter().for_each(|(key, value)| {
        //             match key.to_lowercase().trim() {
        //                 "date" => {
        //                     p.date = Some(value.trim().to_string());
        //                 }
        //                 "id" => {
        //                     p.id = Some(value.trim().to_string());
        //                 }
        //                 "status" => {
        //                     p.status = Some(value.trim().to_string());
        //                 }
        //                 "time" => {
        //                     p.time = Some(value.trim().to_string());
        //                 }
        //                 "type" => {
        //                     p.r#type = Some(value.trim().to_string());
        //                 }
        //                 _ => {}
        //             }
        //             ()
        //         });
        //         ()
        //     }
        //     _ => (),
        // });

        // let filtered_sections: Option<Vec<Section>> = Some(
        //     raw_sections
        //         .into_iter()
        //         .filter_map(|sec| match sec {
        //             Section::RawPageAttributes(key_values) => {
        //                 dbg!("--------------------");
        //                 key_values.iter().for_each(|(key, value)| {
        //                     match key.to_lowercase().trim() {
        //                         "date" => {
        //                             p.date = Some(value.trim().to_string());
        //                         }
        //                         "id" => {
        //                             p.id = Some(value.trim().to_string());
        //                         }
        //                         "status" => {
        //                             p.status = Some(value.trim().to_string());
        //                         }
        //                         "time" => {
        //                             p.time = Some(value.trim().to_string());
        //                         }
        //                         "type" => {
        //                             p.r#type = Some(value.trim().to_string());
        //                         }
        //                         _ => {}
        //                     }
        //                     ()
        //                 });
        //                 None
        //             }
        //             x => Some(x),
        //         })
        //         .collect(),
        // );

        // if filtered_sections.as_ref().unwrap().len() == 0 {
        //     p.sections = None;
        // } else {
        //     // p.sections = filtered_sections;
        //     p.sections = None;
        // }

        //
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reference {}

// pub fn page(source: &str) -> IResult<&str, Page> {
//     let mut p = Page::new();
//     let raw_sections = sections(source).unwrap().1;
//     p.sections = Some(raw_sections);
//     Ok((source, p))
// }
