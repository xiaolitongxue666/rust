use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let english_26_characters: HashSet<char> = 
        HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
    
    // 提取句子中的所有小写字母并去重
    let sentence_letters: HashSet<char> = sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    
    // 检查句子的字母集合是否包含所有26个字母
    sentence_letters == english_26_characters
}
