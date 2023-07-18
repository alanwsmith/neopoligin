use crate::snippets::Snippet;
pub mod paragraph;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    None,
    Paragraph{
        snippets: Vec<Snippet>
    }
}