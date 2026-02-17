//! # Decimal - 任意精度小数
//!
//! 使用 BigInt 存储 digits，decimal_index 表示小数点位置。
//!
//! ## 考点
//! - num_bigint、num_traits
//! - PartialEq、PartialOrd、Add、Sub、Mul

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};

use num_bigint::BigInt;
use num_traits::Pow;

#[derive(Debug, Clone)]
pub struct Decimal {
    digits: BigInt,
    decimal_index: usize,
}

impl Decimal {
    fn new(digits: BigInt, decimal_index: usize) -> Self {
        let mut d = Decimal {
            digits,
            decimal_index,
        };
        d.reduce();
        d
    }

    pub fn try_from(mut input: &str) -> Option<Decimal> {
        input = input.trim();
        let mut digits_str = String::with_capacity(input.len());
        let mut decimal_index = None;
        for ch in input.chars() {
            match ch {
                '0'..='9' | '-' | '+' => {
                    digits_str.push(ch);
                    if let Some(ref mut idx) = decimal_index {
                        *idx += 1;
                    }
                }
                '.' => {
                    if decimal_index.is_some() {
                        return None;
                    }
                    decimal_index = Some(0);
                }
                _ => return None,
            }
        }
        let digits: BigInt = digits_str.parse().ok()?;
        Some(Decimal::new(digits, decimal_index.unwrap_or(0)))
    }

    fn equalize_precision(a: &mut Decimal, b: &mut Decimal) {
        let (lo, hi) = if a.decimal_index < b.decimal_index {
            (a, b)
        } else {
            (b, a)
        };
        let diff = hi.decimal_index - lo.decimal_index;
        lo.digits = &lo.digits * BigInt::from(10).pow(diff);
        lo.decimal_index = hi.decimal_index;
    }

    fn reduce(&mut self) {
        let s = self.digits.to_string();
        let frac_len = self.decimal_index.min(s.len());
        let trailing = s
            .chars()
            .rev()
            .take(frac_len)
            .take_while(|&c| c == '0')
            .count();
        if trailing > 0 {
            self.digits = &self.digits / BigInt::from(10).pow(trailing);
            self.decimal_index -= trailing;
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.clone();
        let mut b = other.clone();
        Decimal::equalize_precision(&mut a, &mut b);
        a.digits == b.digits
    }
}

impl Eq for Decimal {}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut a = self.clone();
        let mut b = other.clone();
        Decimal::equalize_precision(&mut a, &mut b);
        a.digits.cmp(&b.digits)
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(mut self, mut rhs: Self) -> Self {
        Decimal::equalize_precision(&mut self, &mut rhs);
        Decimal::new(self.digits + rhs.digits, self.decimal_index)
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(mut self, mut rhs: Self) -> Self {
        Decimal::equalize_precision(&mut self, &mut rhs);
        Decimal::new(self.digits - rhs.digits, self.decimal_index)
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Decimal::new(
            self.digits * rhs.digits,
            self.decimal_index + rhs.decimal_index,
        )
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.digits.to_string();
        let neg = s.starts_with('-');
        let digits: &str = if neg { &s[1..] } else { &s };
        let width = self.decimal_index;
        let padded = if digits.len() >= width {
            format!("{}{:0>width$}", digits, "", width = width)
        } else {
            format!("{:0>width$}", digits, width = width)
        };
        let (int_part, frac_part) = padded.split_at(padded.len() - width);
        let int_part = int_part.trim_start_matches('0');
        let int_part = if int_part.is_empty() { "0" } else { int_part };
        let frac_part = frac_part.trim_end_matches('0');
        let out = if frac_part.is_empty() {
            format!("{}{}", if neg { "-" } else { "" }, int_part)
        } else {
            format!("{}{}.{}", if neg { "-" } else { "" }, int_part, frac_part)
        };
        write!(f, "{}", out)
    }
}
