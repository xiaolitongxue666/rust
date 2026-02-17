/// 将单词转换为 Pig Latin
///
/// 规则：
/// - 以元音（a, e, i, o, u）开头的单词：在末尾加 "ay"
/// - 以 "xr" 或 "yr" 开头的单词：视为元音，在末尾加 "ay"
/// - 以辅音开头的单词：将开头的辅音（或辅音组合）移到末尾，然后加 "ay"
/// - "qu" 被视为一个辅音单位
/// - "y" 在开头是辅音，在辅音簇后是元音
pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word) // 对每个 &str 元素调用 translate_word
        .collect::<Vec<_>>()
        .join(" ")
}

/// 翻译单个单词
fn translate_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    // 检查是否以元音开头（包括 xr、yr 和 yt 的特殊情况）
    if starts_with_vowel_sound(word) {
        return format!("{}ay", word);
    }

    // 找到第一个元音的位置
    let chars: Vec<char> = word.chars().collect();
    let vowel_index = find_first_vowel(&chars);

    if vowel_index > 0 {
        // 将辅音部分移到末尾
        let consonant_cluster: String = chars[..vowel_index].iter().collect();
        let rest: String = chars[vowel_index..].iter().collect();
        format!("{}{}ay", rest, consonant_cluster)
    } else {
        // 没有找到元音（理论上不应该发生），直接加 ay
        format!("{}ay", word)
    }
}

/// 检查是否以元音音开头
fn starts_with_vowel_sound(word: &str) -> bool {
    if word.is_empty() {
        return false;
    }

    let word_lower = word.to_ascii_lowercase();
    let first_char = word_lower.chars().next().unwrap();

    // 检查是否是元音
    matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u') ||
    // 检查是否是 xr、yr 或 yt 开头
    word_lower.starts_with("xr") || 
    word_lower.starts_with("yr") || 
    word_lower.starts_with("yt")
}

/// 找到第一个元音的位置
fn find_first_vowel(chars: &[char]) -> usize {
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i].to_ascii_lowercase();

        // 检查是否是元音
        if matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u') {
            return i;
        }

        // 检查是否是 y（在辅音簇后被视为元音）
        if ch == 'y' && i > 0 {
            return i;
        }

        // 处理 qu 的情况（qu 被视为一个辅音单位）
        if ch == 'q' && i + 1 < chars.len() && chars[i + 1].to_ascii_lowercase() == 'u' {
            i += 2; // 跳过 qu
            continue;
        }

        i += 1;
    }

    chars.len() // 如果没有找到元音，返回字符串长度
}
