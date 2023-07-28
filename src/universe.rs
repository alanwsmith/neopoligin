use crate::blocks::Block;
use crate::page::Page;
use minijinja::value::{StructObject, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Universe {
    pub pages: Vec<Page>,
    pub cache_full_link_list: Option<Vec<(String, Option<Block>)>>,
}

impl Universe {
    pub fn full_link_list(&mut self) -> Vec<(String, Option<Block>)> {
        if let None = self.cache_full_link_list {
            self.cache_full_link_list = Some(
                self.pages
                    .clone()
                    .into_iter()
                    .map(|p| {
                        (
                            p.path.as_ref().unwrap().display().to_string(),
                            p.clone().title_string(),
                        )
                    })
                    .collect(),
            )
        }
        self.clone().cache_full_link_list.unwrap()
    }
}

impl StructObject for Universe {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "full_link_list" => Some(Value::from_serializable(&self.clone().full_link_list())),
            _ => None,
        }
    }
}
