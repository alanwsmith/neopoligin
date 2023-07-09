use crate::blocks::Block;
use serde::Serialize;

pub mod list_item;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem{ paragraphs: Vec<Block> },
}
