use super::data_classes::{ScalarObject, Value};

pub fn mk_scalar(value: Value) -> Option<ScalarObject> {
    match value {
        Value::Bool(value) => Some(ScalarObject::Logical(Some(value))),
        Value::Integer(value) => Some(ScalarObject::Integer(Some(value))),
        Value::Number(value) => Some(ScalarObject::Numeric(Some(value))),
        Value::Text(value) => Some(ScalarObject::Character(value)),
        _ => None,
    }
}
