pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return vec![];
    }
    let mut result = Vec::new();
    for i in 0..=digits.len() - len {
        result.push(digits[i..i + len].to_string());
    }
    result
}
