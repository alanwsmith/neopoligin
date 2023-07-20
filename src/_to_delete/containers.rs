use crate::blocks::Block;

pub mod checklistitem;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    None,
    ChecklistItem{
        blocks: Option<Vec<Block>>
    }
}