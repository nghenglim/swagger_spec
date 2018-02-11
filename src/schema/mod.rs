extern crate serde_json;
use DataType;
use self::serde_json::Value;
#[derive(Clone, Debug)]
pub struct Schema {
    pub type_: Option<DataType>,
    pub default: Option<Value>,
    pub nullable: bool,
}
impl Schema {
    pub fn new() -> Schema {
        Schema {
            type_: None,
            default: None,
            nullable: false,
        }
    }
    pub fn type_(&mut self, t: DataType) -> &mut Self {
        self.type_ = Some(t);
        self
    }
}