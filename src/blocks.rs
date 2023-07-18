use crate::tags::Tag;
use crate::snippets::Snippet;
use serde::{Deserialize, Serialize};

pub mod headline;
pub mod paragraph;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Headline { tags: Vec<Tag> },
    Paragraph { tags: Vec<Tag> },
    HeadlineNewVersion { snippets: Vec<Snippet> },
    ParagraphNewVersion { snippets: Vec<Snippet> },
}

