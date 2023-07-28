use crate::page::Page;
use minijinja::value::{StructObject, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Universe {
    pub pages: Vec<Page>,
}

impl Universe {
    pub fn home_page_links(&self) -> Vec<(String, String)> {
        vec![("a".to_string(), "b".to_string())]
    }
}

impl StructObject for Universe {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "home_page_links" => Some(Value::from_serializable(&self.clone().home_page_links())),
            _ => None,
        }
    }
}
