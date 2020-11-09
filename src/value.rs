use std::collections::BTreeMap;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Int(i64),
    Str(String),
    List(Vec<Value>),
    Dict(BTreeMap<String, Value>),
}
