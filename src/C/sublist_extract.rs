use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SimplifiedValue {
    List(Vec<Value>),
    Integer(Vec<i32>),
    Double(Vec<f64>),
    Logical(Vec<Option<bool>>),
    Complex(Vec<(f64, f64)>),
    String(Vec<String>),
    Raw(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Double(f64),
    Logical(Option<bool>),
    Complex(f64, f64),
    String(String),
    Raw(u8),
    Vector(Vec<Value>),
    List(BTreeMap<String, Value>),
}

pub type NamedList = BTreeMap<String, Value>;

pub fn sublist_extract(
    values: &[NamedList],
    name: &str,
    simplify: bool,
    keep_names: bool,
) -> Result<SimplifiedValue, String> {
    if name.is_empty() {
        return Err("'name' cannot be empty".to_string());
    }

    let extracted = values
        .iter()
        .enumerate()
        .map(|(idx, list)| {
            list.get(name)
                .cloned()
                .ok_or_else(|| format!("no element named '{name}' at index {idx}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let _ = keep_names;
    if !simplify {
        return Ok(SimplifiedValue::List(extracted));
    }
    simplify_values(extracted)
}

fn simplify_values(values: Vec<Value>) -> Result<SimplifiedValue, String> {
    if values.is_empty() {
        return Err("can't extract from an empty list when simplify=TRUE".to_string());
    }

    match &values[0] {
        Value::Integer(_) => values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| match value {
                Value::Integer(value) => Ok(value),
                _ => Err(type_error(idx)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(SimplifiedValue::Integer),
        Value::Double(_) => values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| match value {
                Value::Double(value) => Ok(value),
                _ => Err(type_error(idx)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(SimplifiedValue::Double),
        Value::Logical(_) => values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| match value {
                Value::Logical(value) => Ok(value),
                _ => Err(type_error(idx)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(SimplifiedValue::Logical),
        Value::Complex(_, _) => values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| match value {
                Value::Complex(real, imaginary) => Ok((real, imaginary)),
                _ => Err(type_error(idx)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(SimplifiedValue::Complex),
        Value::String(_) => values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| match value {
                Value::String(value) => Ok(value),
                _ => Err(type_error(idx)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(SimplifiedValue::String),
        Value::Raw(_) => values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| match value {
                Value::Raw(value) => Ok(value),
                _ => Err(type_error(idx)),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(SimplifiedValue::Raw),
        _ => Err("unable to simplify when extracted type is not scalar".to_string()),
    }
}

fn type_error(idx: usize) -> String {
    format!("unable to simplify, element {idx} has incompatible type")
}
