use serde::Serialize;
use std::path::PathBuf;

pub mod content;
pub mod date;
pub mod new;
pub mod template;
pub mod title;
pub mod type_of_page;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SourceFile {
    pub source_data: String,
    pub source_path: PathBuf,
    pub url: String,
}
