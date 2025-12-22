/// 解析并计算简单的数学文字问题
/// 
/// 支持的格式：
/// - "What is 5?" -> Some(5)
/// - "What is 1 plus 1?" -> Some(2)
/// - "What is 1 plus 5 minus -2?" -> Some(8)
/// - "What is -3 multiplied by 25?" -> Some(-75)
/// - "What is 33 divided by -3?" -> Some(-11)
/// - "What is 2 raised to the 5th power?" -> Some(32)
/// 
/// 注意：从左到右计算，忽略运算符优先级
pub fn answer(command: &str) -> Option<i32> {
    /// 从序数词中提取数字（如 "2nd" -> 2, "5th" -> 5）
    fn parse_ordinal(ordinal: &str) -> Option<i32> {
        // 移除序数后缀（st, nd, rd, th）
        let num_str = ordinal
            .trim_end_matches("st")
            .trim_end_matches("nd")
            .trim_end_matches("rd")
            .trim_end_matches("th");
        num_str.parse::<i32>().ok()
    }
    // 移除末尾的问号
    let command = command.trim_end_matches('?');
    
    // 分割为单词
    let words: Vec<&str> = command.split_whitespace().collect();
    
    // 必须至少包含 "What is <number>"
    if words.len() < 3 {
        return None;
    }
    
    // 检查开头是否为 "What is"
    if words[0] != "What" || words[1] != "is" {
        return None;
    }
    
    // 解析第一个数字
    let mut result = words[2].parse::<i32>().ok()?;
    let mut index = 3;
    
    // 如果只有数字，直接返回
    if index >= words.len() {
        return Some(result);
    }
    
    // 处理后续的操作和数字
    while index < words.len() {
        // 获取操作符
        let op = words.get(index)?;
        
        match *op {
            "plus" => {
                index += 1;
                if index >= words.len() {
                    return None; // 缺少操作数
                }
                let num = words[index].parse::<i32>().ok()?;
                result += num;
                index += 1;
            }
            "minus" => {
                index += 1;
                if index >= words.len() {
                    return None; // 缺少操作数
                }
                let num = words[index].parse::<i32>().ok()?;
                result -= num;
                index += 1;
            }
            "multiplied" => {
                index += 1;
                if index >= words.len() || words[index] != "by" {
                    return None; // 缺少 "by"
                }
                index += 1;
                if index >= words.len() {
                    return None; // 缺少操作数
                }
                let num = words[index].parse::<i32>().ok()?;
                result *= num;
                index += 1;
            }
            "divided" => {
                index += 1;
                if index >= words.len() || words[index] != "by" {
                    return None; // 缺少 "by"
                }
                index += 1;
                if index >= words.len() {
                    return None; // 缺少操作数
                }
                let num = words[index].parse::<i32>().ok()?;
                if num == 0 {
                    return None; // 除零错误
                }
                result /= num;
                index += 1;
            }
            "raised" => {
                // 处理 "raised to the Xth power" 格式
                index += 1; // 跳过 "raised"
                if index >= words.len() || words[index] != "to" {
                    return None; // 缺少 "to"
                }
                index += 1; // 跳过 "to"
                if index >= words.len() || words[index] != "the" {
                    return None; // 缺少 "the"
                }
                index += 1; // 跳过 "the"
                if index >= words.len() {
                    return None; // 缺少指数
                }
                // 解析指数（序数词，如 "2nd", "5th"）
                let exponent = parse_ordinal(words[index])?;
                index += 1; // 跳过序数词
                if index >= words.len() || words[index] != "power" {
                    return None; // 缺少 "power"
                }
                index += 1; // 跳过 "power"
                
                // 计算 result^exponent
                result = result.pow(exponent as u32);
            }
            _ => {
                // 未知操作或无效输入
                return None;
            }
        }
    }
    
    Some(result)
}
