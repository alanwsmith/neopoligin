use crate::blocks::Block;
use serde::{Deserialize, Serialize};
use crate::sections::todo::TodoStatus;

pub mod list_item;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem{ paragraphs: Vec<Block> },
    TodoItem{ 
        status: TodoStatus,
        paragraphs: Vec<Block> 
    },
}
