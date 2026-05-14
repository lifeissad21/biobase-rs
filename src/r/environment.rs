use std::collections::BTreeMap;

pub type Environment<T> = BTreeMap<String, T>;

pub fn multiassign<T: Clone>(
    names: &[String],
    values: &[T],
    env: &mut Environment<T>,
) -> Result<(), String> {
    if names.len() != values.len() {
        return Err("names and values must have the same length".to_string());
    }
    for (name, value) in names.iter().zip(values.iter()) {
        env.insert(name.clone(), value.clone());
    }
    Ok(())
}

pub fn contents<T>(env: &Environment<T>) -> Vec<&str> {
    env.keys().map(String::as_str).collect()
}

pub fn list_len<T>(values: &[Vec<T>]) -> Vec<usize> {
    values.iter().map(Vec::len).collect()
}

pub fn copy_env_with_all_names<T: Clone>(
    old_env: &Environment<T>,
    new_env: &mut Environment<T>,
    all_names: bool,
) {
    new_env.extend(
        old_env
            .iter()
            .filter(|(key, _)| all_names || !key.starts_with('.'))
            .map(|(key, value)| (key.clone(), value.clone())),
    );
}

pub fn copy_env<T: Clone>(old_env: &Environment<T>, new_env: &mut Environment<T>) {
    copy_env_with_all_names(old_env, new_env, false);
}
