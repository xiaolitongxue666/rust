const ALLOWED_NON_DIGITS: &[char] = &[' ', '(', ')', '-', '.', '+'];

pub fn number(user_number: &str) -> Option<String> {
    let mut digits = String::new();
    for c in user_number.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else if !ALLOWED_NON_DIGITS.contains(&c) {
            return None;
        }
    }

    let digits: Vec<char> = digits.chars().collect();
    let len = digits.len();

    let ten_digits = if len == 11 && digits[0] == '1' {
        &digits[1..]
    } else if len == 10 {
        digits.as_slice()
    } else {
        return Non
    };

    let area_first = ten_digits[0];
    let exchange_first = ten_digits[3];

    if matches!(area_first, '0' | '1') || matches!(exchange_first, '0' | '1') {
        return None;
    }

    Some(ten_digits.iter().collect())
}
