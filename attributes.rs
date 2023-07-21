

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Attribute {

}