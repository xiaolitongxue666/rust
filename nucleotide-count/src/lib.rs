//! DNA 核苷酸计数：统计 A/C/G/T 各碱基出现次数
//!
//! 考点：Result、HashMap、matches!、filter、? 操作符

use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    // 验证nucleotide是否为有效的DNA碱基
    if !is_valid_nucleotide(nucleotide) {
        return Err(nucleotide);
    }

    // 验证dna字符串中的所有字符
    for c in dna.chars() {
        if !is_valid_nucleotide(c) {
            return Err(c);
        }
    }

    Ok(dna.chars().filter(|c| *c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // 验证dna字符串中的所有字符
    for c in dna.chars() {
        if !is_valid_nucleotide(c) {
            return Err(c);
        }
    }

    let mut counts = HashMap::new();
    counts.insert('A', count('A', dna)?);
    counts.insert('C', count('C', dna)?);
    counts.insert('G', count('G', dna)?);
    counts.insert('T', count('T', dna)?);
    Ok(counts)
}

fn is_valid_nucleotide(c: char) -> bool {
    matches!(c, 'A' | 'C' | 'G' | 'T')
}
