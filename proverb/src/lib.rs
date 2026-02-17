//! 谚语生成：For want of a X the Y was lost. And all for the want of a 首词.
//!
//! 考点：切片迭代、相邻元素配对、字符串拼接

pub fn build_proverb(list: &[&str]) -> String {
    let mut iter = list.iter();
    let first_iterm;
    let if_empty_iter = iter.next();
    match if_empty_iter {
        None => {
            return "".to_string();
        }
        Some(x) => first_iterm = x,
    }

    println!(" Iterm is {} .", first_iterm);
    let mut first_flug = true;
    let mut second_iterm;
    let mut in_for_first_iterm = first_iterm;
    let mut result_string: String = "".to_string();
    for val in iter {
        second_iterm = &val;
        println!(" Iterm in for is {} .", second_iterm);
        if first_flug {
            result_string.push_str("For want of a ");
            first_flug = false;
        } else {
            result_string.push_str("\nFor want of a ")
        }
        result_string.push_str(in_for_first_iterm);
        result_string.push_str(" the ");
        result_string.push_str(second_iterm);
        result_string.push_str(" was lost.");
        in_for_first_iterm = second_iterm;
    }
    if first_flug {
        result_string.push_str("And all for the want of a ");
    } else {
        result_string.push_str("\nAnd all for the want of a ");
    }
    result_string.push_str(first_iterm);
    result_string.push_str(".");

    result_string
}
