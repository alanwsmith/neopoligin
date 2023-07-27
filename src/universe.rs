use crate::page::Page;
use minijinja::value::{StructObject, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Universe {
    pub pages: Vec<Page>,
    // pub all_links_cache: Option<Vec<Link>>,
    // pub rumba: Option<String>,
    // pub alfa: Option<String>,
}

impl Universe {
    pub fn ping(&self) -> String {
        "HELLLLLLLLO".to_string()
    }
    pub fn rumba(&self) -> String {
        "RUUUUUUUUMMMMMMMBA".to_string()
    }
}

impl StructObject for Universe {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "ping" => Some(Value::from(self.ping())),
            "rumba" => Some(Value::from(self.rumba())),
            _ => None,
        }
    }
}

// impl Universe {
//     pub fn all_links(&mut self) -> Vec<Link> {
//         match self.all_links_cache.clone() {
//             None => {
//                 self.all_links_cache = Some(
//                     self.pages
//                         .clone()
//                         .into_iter()
//                         .map(|page| Link::Internal {
//                             url: page.path.as_ref().unwrap().display().to_string(),
//                             text: page.path.as_ref().unwrap().display().to_string(),
//                         })
//                         .collect(),
//                 );
//                 self.all_links_cache.clone().unwrap()
//             }
//             Some(cache) => cache,
//         }
//     }
// }
