use std::collections::HashMap;

/// 解析输入字符串，分割成左边单词列表和右边结果单词
/// 输入: "I + BB == ILL"
/// 输出: (["I", "BB"], "ILL")
fn parse_equation(input: &str) -> (Vec<&str>, &str) {
    let parts: Vec<&str> = input.split("==").collect();
    let left_side: Vec<&str> = parts[0].split('+').map(|s| s.trim()).collect();
    let right_side = parts[1].trim();
    (left_side, right_side)
}

/// 从等式中提取所有唯一的字母，按字母顺序排序
/// 输入: "I + BB == ILL"
/// 输出: ['B', 'I', 'L']
fn extract_letters(equation: &str) -> Vec<char> {
    let mut letters: Vec<char> = equation
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    letters.sort();
    letters
}

/// 将单词转换为数字
/// 根据字母到数字的映射，将单词转换为对应的数值
/*
初始值: acc = 0
第1次迭代: acc = 0 * 10 + 9 = 9
第2次迭代: acc = 9 * 10 + 5 = 95  
第3次迭代: acc = 95 * 10 + 6 = 956
第4次迭代: acc = 956 * 10 + 7 = 9567
*/
fn word_to_number(word: &str, assignment: &HashMap<char, u8>) -> u64 {
    word.chars()
        .map(|c| assignment[&c] as u64)
        .fold(0, |acc, digit| acc * 10 + digit)
}

/// 检查当前赋值是否违反约束条件
/// 1. 每个字母必须对应不同的数字
/// 2. 多位数的首位不能为0
fn is_valid_assignment(
    assignment: &HashMap<char, u8>,
    left_words: &[&str],
    right_word: &str,
) -> bool {
    // 检查是否有重复的数字分配
    let values: Vec<u8> = assignment.values().cloned().collect();
    let unique_values: Vec<u8> = values
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    if values.len() != unique_values.len() {
        return false;
    }

    // 检查多位数的首位不能为0
    for word in left_words {
        if word.len() > 1
            && let Some(&first_digit) = assignment.get(&word.chars().next().unwrap())
            && first_digit == 0
        {
            return false;
        }
    }

    if right_word.len() > 1
        && let Some(&first_digit) = assignment.get(&right_word.chars().next().unwrap())
        && first_digit == 0
    {
        return false;
    }

    true
}

/// 检查当前赋值是否构成有效的数学等式
fn is_valid_solution(
    assignment: &HashMap<char, u8>,
    left_words: &[&str],
    right_word: &str,
) -> bool {
    if !is_valid_assignment(assignment, left_words, right_word) {
        return false;
    }

    // 计算左边所有单词的数字之和
    let left_sum: u64 = left_words
        .iter()
        .map(|word| word_to_number(word, assignment))
        .sum();

    // 计算右边的数字
    let right_number = word_to_number(right_word, assignment);

    // 检查等式是否成立
    left_sum == right_number
}

/// 回溯算法的主函数
/// 尝试为每个字母分配数字，直到找到有效解或确定无解
/// 
/// 回溯算法核心思想：
/// 1. 系统性地尝试所有可能的数字分配组合
/// 2. 当发现当前路径不可能产生解时，立即回退
/// 3. 通过约束传播和剪枝大幅减少搜索空间
/// 
/// 算法流程：
/// 1. 基础情况：所有字母都已分配数字 → 验证是否为有效解
/// 2. 递归情况：为当前字母尝试所有可能的数字(0-9)
/// 3. 约束检查：确保数字唯一性，避免重复分配
/// 4. 递归探索：尝试下一个字母的所有可能分配
/// 5. 回溯机制：当前路径无解时，撤销分配并尝试下一个数字
fn backtrack(
    letters: &[char],                    // 所有需要分配数字的字母列表
    assignment: &mut HashMap<char, u8>,   // 当前已分配的字母→数字映射
    left_words: &[&str],                 // 等式左边的所有单词
    right_word: &str,                    // 等式右边的结果单词
    letter_index: usize,                 // 当前正在处理的字母索引
) -> Option<HashMap<char, u8>> {
    
    // ========== 基础情况：递归终止条件 ==========
    // 如果所有字母都已分配数字，检查当前分配是否构成有效解
    if letter_index == letters.len() {
        // 验证当前完整分配是否满足所有约束条件：
        // 1. 数字唯一性：每个字母对应不同数字
        // 2. 前导零约束：多位数字首位不能为0
        // 3. 数学等式：左边所有单词的数字之和 = 右边数字
        if is_valid_solution(assignment, left_words, right_word) {
            // 找到有效解！返回完整的字母→数字映射
            return Some(assignment.clone());
        }
        // 当前分配不构成有效解，返回None表示此路径无解
        return None;
    }

    // ========== 递归情况：尝试当前字母的所有可能数字 ==========
    let current_letter = letters[letter_index];
    
    // 为当前字母尝试所有可能的数字(0-9)
    for digit in 0..=9 {
        // ========== 约束检查：数字唯一性 ==========
        // 检查当前数字是否已经被其他字母使用
        // 这是关键的剪枝策略：避免重复数字分配
        if !assignment.values().any(|&v| v == digit) {
            // ========== 做出选择：分配数字 ==========
            // 将当前字母与数字建立映射关系
            assignment.insert(current_letter, digit);
            
            // ========== 递归探索：尝试下一个字母 ==========
            // 递归调用，处理下一个字母的所有可能分配
            // 这里体现了回溯的"深度优先搜索"特性
            if let Some(result) = backtrack(
                letters,                    // 字母列表不变
                assignment,                 // 传递当前分配状态
                left_words,                 // 等式信息不变
                right_word,                 // 等式信息不变
                letter_index + 1,          // 处理下一个字母
            ) {
                // ========== 找到解：立即返回 ==========
                // 如果递归调用找到了有效解，立即返回结果
                // 这避免了继续搜索其他可能解，提高效率
                return Some(result);
            }
            
            // ========== 回溯机制：撤销选择 ==========
            // 如果当前路径没有找到解，必须撤销刚才的分配
            // 这是回溯算法的核心：当发现死路时，回退到上一个状态
            assignment.remove(&current_letter);
        }
        // 如果当前数字已被使用，直接尝试下一个数字
        // 这实现了约束传播：避免无效的搜索分支
    }
    
    // ========== 无解情况：所有数字都尝试失败 ==========
    // 如果当前字母的所有可能数字(0-9)都无法产生有效解
    // 返回None，让上层递归尝试其他路径
    None
}

/// 主求解函数
/// 解析输入，提取字母，使用回溯算法寻找解
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // 1. 分割字符串，获取左边单词列表和右边结果单词
    let (left_words, right_word) = parse_equation(input);

    // 2. 提取所有唯一的字母
    let letters = extract_letters(input);

    // 3. 如果只有一个字母，直接处理特殊情况
    if letters.len() == 1 {
        let letter = letters[0];
        let mut assignment = HashMap::new();
        assignment.insert(letter, 1);
        if is_valid_solution(&assignment, &left_words, right_word) {
            return Some(assignment);
        }
        return None;
    }

    // 4. 使用回溯算法寻找解
    let mut assignment = HashMap::new();
    backtrack(&letters, &mut assignment, &left_words, right_word, 0)
}
