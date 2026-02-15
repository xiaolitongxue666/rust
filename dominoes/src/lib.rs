//! # Dominoes - 多米诺链
//!
//! 能否将骨牌首尾相连成环。回溯枚举，检查每数出现次数为偶（欧拉回路）。
//!
//! ## 考点
//! - 图论、回溯

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    let mut count = [0u8; 7];
    for &(a, b) in input {
        count[a as usize] += 1;
        count[b as usize] += 1;
    }
    for c in &count[1..7] {
        if *c % 2 != 0 {
            return None;
        }
    }
    let mut used = vec![false; input.len()];
    let mut path = Vec::new();
    if backtrack(input, &mut used, &mut path, None) {
        Some(path)
    } else {
        None
    }
}

fn backtrack(
    input: &[(u8, u8)],
    used: &mut [bool],
    path: &mut Vec<(u8, u8)>,
    need: Option<u8>,
) -> bool {
    if path.len() == input.len() {
        return need == Some(path[0].0);
    }
    for (i, &(a, b)) in input.iter().enumerate() {
        if used[i] {
            continue;
        }
        let (dom, next) = match need {
            None => {
                used[i] = true;
                path.push((a, b));
                if backtrack(input, used, path, Some(b)) {
                    return true;
                }
                path.pop();
                used[i] = false;
                if a != b {
                    used[i] = true;
                    path.push((b, a));
                    if backtrack(input, used, path, Some(a)) {
                        return true;
                    }
                    path.pop();
                    used[i] = false;
                }
                continue;
            }
            Some(n) if n == a => ((a, b), b),
            Some(n) if n == b => ((b, a), a),
            _ => continue,
        };
        used[i] = true;
        path.push(dom);
        if backtrack(input, used, path, Some(next)) {
            return true;
        }
        path.pop();
        used[i] = false;
    }
    false
}
