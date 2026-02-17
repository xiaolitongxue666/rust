//! 闰年判断：400 整除必闰；100 整除不闰；4 整除则闰
//!
//! 考点：布尔逻辑、取模运算

pub fn is_leap_year(year: u64) -> bool {
    if year % 400 == 0 {
        true
    } else {
        if year % 100 == 0 {
            false
        } else {
            if year % 4 == 0 {
                true
            } else {
                false
            }
        }
    }
}
