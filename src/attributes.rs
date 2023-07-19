use serde::{Deserialize, Serialize};
pub mod accesskey;
use crate::attributes::accesskey::accesskey;
pub mod autocapitalize;
use crate::attributes::autocapitalize::autocapitalize;
pub mod autofocus;
use crate::attributes::autofocus::autofocus;
use nom::multi::many0;
use nom::IResult;
use nom::branch::alt;

pub fn attributes(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (source, attributes) = many0(alt((accesskey, autocapitalize, autofocus)))(source)?;
    if attributes.len() == 0 {
        Ok((source, None))
    } else {
        Ok((source, Some(attributes)))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Attribute {
    // these are global
    AccessKey(String),
    AutoCapitalize(AutoCapitalizeValue),
    AutoFocus,
    Class(Vec<String>),
    ContentEditable(ContentEditableAttrOption),
    Data(String, String),
    Dir(DirAttrOption),
    Draggable(bool),
    EnterKeyHint(String),
    Hidden(Option<HiddenAttrOption>),
    Id(String),
    Inert(),
    InputMode(InputModeAttrOption),
    Is(String),
    ItemId(String),
    ItemProp(String),
    ItemRef(String),
    ItemScope(String),
    ItemType(String),
    Lang(String),
    Nonce(String),
    Part(Vec<String>),
    Popover(PopoverAttrOption),
    PopoverTarget(String),
    Spellcheck(bool),
    Style(Vec<(String, String)>),
    TabIndex(i32), // can be negative
    Title(String),
    Translate(TranslateAttrOption),
    VirtualKeyboardPolicy(VirtualKeyboardPolicyAttrOption),
    // these are specific
    Accept(),
    AutoComplete(),
    Capture(),
    CrossOrigin(),
    Disabled(),
    ElementTiming(),
    For(),
    Max(),
    MaxLength(),
    Min(),
    MinLength(),
    Multiple(),
    Pattern(),
    ReadOnly(),
    Rel(),
    Required(),
    Size(),
    Step(),
    None, // <- for dev testing
}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum AutoCapitalizeValue {
    Off,
    None,
    On,
    Sentences,
    Words,
    Characters,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum ContentEditableAttrOption {
    True(),
    False(),
    PlaintextOnly(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum DirAttrOption {
    Ltr(),
    Rtl(),
    Auto(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum HiddenAttrOption {
    Hidden(),
    UntilFound(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum InputModeAttrOption {
    None(),
    Text(),
    Decimal(),
    Numeric(),
    Tel(),
    Search(),
    Email(),
    Url(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum PopoverAttrOption {
    Auto(),
    Manual(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum TranslateAttrOption {
    Yes(),
    No(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum VirtualKeyboardPolicyAttrOption {
    Auto(),
    Manual(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum InputTypes {
    Button(),
    Checkbox(),
    Color(),
    Date(),
    DatatimeLocal(),
    Email(),
    File(),
    Hidden(),
    Image(),
    Month(),
    Number(),
    Password(),
    Radio(),
    Range(),
    Reset(),
    Search(),
    Submit(),
    Tel(),
    Text(),
    Time(),
    Url(),
    Week(),
}
