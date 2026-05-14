use std::collections::{BTreeMap, BTreeSet};

pub fn reverse_split(input: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, Vec<String>> {
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, values) in input {
        for value in values {
            out.entry(value.clone()).or_default().push(name.clone());
        }
    }
    out
}

pub fn note(message: impl AsRef<str>) -> String {
    format!("Note: {}\n", message.as_ref())
}

pub fn is_unique<T>(values: &[T]) -> Vec<bool>
where
    T: Ord + Clone,
{
    let mut counts = BTreeMap::<T, usize>::new();
    for value in values {
        *counts.entry(value.clone()).or_default() += 1;
    }
    values.iter().map(|value| counts[value] == 1).collect()
}

pub fn valid_msg(mut msg: Vec<String>, result: Result<(), Vec<String>>) -> Vec<String> {
    if let Err(mut errors) = result {
        msg.append(&mut errors);
    }
    msg
}

pub fn check_class(actual: &str, expected: &[&str]) -> Result<(), String> {
    if expected.contains(&actual) {
        Ok(())
    } else {
        Err(format!(
            "'object' is class '{actual}' but should be or extend '{}'",
            expected.join("', '")
        ))
    }
}

pub fn user_query_default(default: &str, allowed: &[&str]) -> String {
    if allowed.contains(&default) {
        default.to_string()
    } else {
        allowed.first().copied().unwrap_or(default).to_string()
    }
}

pub fn sub_list_extract<T: Clone>(
    values: &[BTreeMap<String, T>],
    name: &str,
) -> Result<Vec<T>, String> {
    values
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            entry
                .get(name)
                .cloned()
                .ok_or_else(|| format!("bad inner list at index {idx}, no element named {name}"))
        })
        .collect()
}

pub fn list_len<T>(values: &[Vec<T>]) -> Vec<usize> {
    values.iter().map(Vec::len).collect()
}

pub fn select_some<T: Clone>(values: &[T], max_to_show: usize) -> Vec<T> {
    values.iter().take(max_to_show).cloned().collect()
}

pub fn duplicated<T>(values: &[T]) -> Vec<bool>
where
    T: Ord + Clone,
{
    let mut seen = BTreeSet::new();
    values
        .iter()
        .map(|value| !seen.insert(value.clone()))
        .collect()
}
