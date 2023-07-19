use serde::{Deserialize, Serialize};
pub mod accesskey;
use crate::attrs::accesskey::accesskey;
pub mod autocapitalize;
use crate::attrs::autocapitalize::autocapitalize;
pub mod autofocus;
use crate::attrs::autofocus::autofocus;
pub mod generic;
use crate::attrs::generic::generic;
pub mod height;
use crate::attrs::height::height;
pub mod width;
use crate::attrs::width::width;
use nom::multi::many0;
use nom::IResult;
use nom::branch::alt;

pub fn attrs(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    dbg!(&source);
    let (source, attributes) = many0(
        alt((
            alt((accesskey, autocapitalize, autofocus, height, width)),
            generic
        ))
    )(source)?;
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
    // Class(Vec<String>),
    // ContentEditable(ContentEditableAttrOption),
    // Data(String, String),
    // Dir(DirAttrOption),
    // Draggable(bool),
    // EnterKeyHint(String),
    // Hidden(Option<HiddenAttrOption>),
    Generic(String, String),
    // Id(String),
    // Inert(),
    // InputMode(InputModeAttrOption),
    // Is(String),
    // ItemId(String),
    // ItemProp(String),
    // ItemRef(String),
    // ItemScope(String),
    // ItemType(String),
    // Lang(String),
    // Nonce(String),
    // Part(Vec<String>),
    // Popover(PopoverAttrOption),
    // PopoverTarget(String),
    // Spellcheck(bool),
    // Style(Vec<(String, String)>),
    // TabIndex(i32), // can be negative
    // Title(String),
    // Translate(TranslateAttrOption),
    // VirtualKeyboardPolicy(VirtualKeyboardPolicyAttrOption),
    // these are specific
    // Accept(),
    // AutoComplete(),
    // Capture(),
    // CrossOrigin(),
    // Disabled(),
    // ElementTiming(),
    // For(),
    Height(u32),
    // Max(),
    // MaxLength(),
    // Min(),
    // MinLength(),
    // Multiple(),
    // Pattern(),
    // ReadOnly(),
    // Rel(),
    // Required(),
    // Size(),
    // Step(),
    Width(u32),
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

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
// pub enum ContentEditableAttrOption {
//     True(),
//     False(),
//     PlaintextOnly(),
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
// pub enum DirAttrOption {
//     Ltr(),
//     Rtl(),
//     Auto(),
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
// pub enum HiddenAttrOption {
//     Hidden(),
//     UntilFound(),
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
// pub enum InputModeAttrOption {
//     None(),
//     Text(),
//     Decimal(),
//     Numeric(),
//     Tel(),
//     Search(),
//     Email(),
//     Url(),
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
// pub enum PopoverAttrOption {
//     Auto(),
//     Manual(),
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
// pub enum TranslateAttrOption {
//     Yes(),
//     No(),
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
// pub enum VirtualKeyboardPolicyAttrOption {
//     Auto(),
//     Manual(),
// }

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", content = "content", rename_all = "lowercase")]
// pub enum InputTypes {
//     Button(),
//     Checkbox(),
//     Color(),
//     Date(),
//     DatatimeLocal(),
//     Email(),
//     File(),
//     Hidden(),
//     Image(),
//     Month(),
//     Number(),
//     Password(),
//     Radio(),
//     Range(),
//     Reset(),
//     Search(),
//     Submit(),
//     Tel(),
//     Text(),
//     Time(),
//     Url(),
//     Week(),
// }
