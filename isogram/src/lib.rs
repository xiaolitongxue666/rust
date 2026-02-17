//! 等值线词：判断单词中是否有重复字母（忽略大小写）
//!
//! 考点：chars、is_ascii_alphabetic、迭代器 clone/find、大小写不敏感比较

pub fn check(candidate: &str) -> bool {
    if candidate.len() <= 1 {
        return true;
    }

    println!("String is {} .", candidate);
    let mut chars_iter = candidate.chars();

    loop {
        if let Some(x) = chars_iter.next() {
            if x.is_ascii_alphabetic() {
                println!(" letter is [{}] .", x);

                let mut temp_iter = chars_iter.clone();
                if let Some(_y) = temp_iter.find(|&letter| letter == x) {
                    println!("Find 01 double {} .", x);
                    return false;
                } else {
                    if x.is_lowercase() {
                        if let Some(_y) = candidate
                            .chars()
                            .find(|&letter| letter == x.to_ascii_uppercase())
                        {
                            println!("Find 02 double {} .", x.to_ascii_uppercase());
                            return false;
                        } else {
                            continue;
                        }
                    }

                    if x.is_uppercase() {
                        if let Some(_y) = candidate
                            .chars()
                            .find(|&letter| letter == x.to_ascii_lowercase())
                        {
                            println!("Find 03 double {} .", x.to_ascii_uppercase());
                            return false;
                        } else {
                            continue;
                        }
                    }
                }
            } else {
                continue;
            } //not a alphabetic
        } else {
            break; //at the end of str
        }
    }
    return true;
}
