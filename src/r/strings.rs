pub fn strbreak(
    input: &str,
    width: usize,
    exdent: usize,
    collapse: &str,
) -> Result<String, String> {
    if width <= 1 {
        return Err("invalid argument 'width'".to_string());
    }
    if exdent > width {
        return Err("invalid argument 'exdent'".to_string());
    }
    if input.chars().count() <= width {
        return Ok(input.to_string());
    }

    let continuation = format!("{collapse}{}", " ".repeat(exdent));
    let mut parts = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut start = 0;
    let mut span = width;
    while start < chars.len() {
        let end = (start + span).min(chars.len());
        parts.push(chars[start..end].iter().collect::<String>());
        start = end;
        span = width - exdent;
    }
    Ok(parts.join(&continuation))
}

pub fn lc_suffix(values: &[impl AsRef<str>], ignore_case: bool) -> String {
    if values.is_empty() {
        return String::new();
    }
    let values = normalize(values, ignore_case);
    let reversed: Vec<Vec<char>> = values
        .iter()
        .map(|value| value.chars().rev().collect())
        .collect();
    let min_len = reversed.iter().map(Vec::len).min().unwrap_or(0);
    let mut len = 0;
    for idx in 0..min_len {
        let first = reversed[0][idx];
        if reversed.iter().all(|chars| chars[idx] == first) {
            len += 1;
        } else {
            break;
        }
    }
    values[0]
        .chars()
        .rev()
        .take(len)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

pub fn lc_prefix(values: &[impl AsRef<str>], ignore_case: bool) -> String {
    if values.is_empty() {
        return String::new();
    }
    let values = normalize(values, ignore_case);
    let chars: Vec<Vec<char>> = values.iter().map(|value| value.chars().collect()).collect();
    let min_len = chars.iter().map(Vec::len).min().unwrap_or(0);
    let mut len = 0;
    for idx in 0..min_len {
        let first = chars[0][idx];
        if chars.iter().all(|value| value[idx] == first) {
            len += 1;
        } else {
            break;
        }
    }
    chars[0].iter().take(len).collect()
}

pub fn lc_prefix_c(values: &[impl AsRef<str>], ignore_case: bool) -> String {
    lc_prefix(values, ignore_case)
}

fn normalize(values: &[impl AsRef<str>], ignore_case: bool) -> Vec<String> {
    values
        .iter()
        .map(|value| {
            if ignore_case {
                value.as_ref().to_uppercase()
            } else {
                value.as_ref().to_string()
            }
        })
        .collect()
}
