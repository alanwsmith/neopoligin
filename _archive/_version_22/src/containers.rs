

use crate::blocks::Block;
























use serde::{Deserialize, Serialize};



pub mod list_item_container;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem { content: Vec<Block> },
    None,
}
