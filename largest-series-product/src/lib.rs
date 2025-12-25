#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    if span == 0 {
        return Ok(1);
    }

    for c in string_digits.chars() {
        if c.to_digit(10).is_none() {
            return Err(Error::InvalidDigit(c));
        }
    }

    let mut star_index = 0;
    let mut max_product = 0;

    while star_index + span <= string_digits.len() {
        let series = &string_digits[star_index..star_index + span];
        let product = series.chars().map(|c| c.to_digit(10).unwrap()).product();
        if product > max_product {
            max_product = product;
        }
        star_index += 1;
    }
    
    Ok(max_product.into())
    
}
