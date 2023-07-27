#![allow(unused_imports)]
use crate::link::Link;
use crate::page::Page;
use minijinja::value::{StructObject, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Payload {}

impl Payload {
    pub fn ping_pay(&self) -> String {
        "Money Money Money".to_string()
    }
}

impl StructObject for Payload {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "ping_pay" => Some(Value::from(self.ping_pay())),
            _ => None,
        }
    }
}
