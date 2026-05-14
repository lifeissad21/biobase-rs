use super::data_classes::Value;

pub fn any_missing(values: &[Value]) -> bool {
    values.iter().any(Value::is_missing)
}
