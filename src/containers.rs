use crate::blocks::Block;
use serde::{Deserialize, Serialize};

pub mod checklist_item_container;
pub mod list_item_container;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem { body: Vec<Block> },
    ChecklistItem { body: Vec<Block> },
    None,
}
