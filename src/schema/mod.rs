extern crate serde_json;
use DataType;
use std::collections::HashMap;
use self::serde_json::Value;
#[derive(Clone, Debug)]
pub struct Schema {
    pub type_: Option<DataType>,
    pub properties: Option<HashMap<String, Schema>>,
    pub default: Option<Value>,
    pub required: Option<Vec<String>>,
    pub nullable: bool,
}
impl Schema {
    pub fn new() -> Schema {
        Schema {
            type_: None,
            default: None,
            nullable: false,
            properties: None,
            required: None,
        }
    }
    pub fn type_(&mut self, t: DataType) -> &mut Self {
        self.type_ = Some(t);
        self
    }
}