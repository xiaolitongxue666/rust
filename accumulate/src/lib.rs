/// 对集合中的每个元素应用函数，返回新的集合
/// 
/// # 参数
/// * `input` - 输入的向量
/// * `function` - 应用于每个元素的函数
/// 
/// # 返回
/// 包含应用函数后结果的新向量
pub fn map<T, U, F>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut result = Vec::with_capacity(input.len());
    for item in input {
        result.push(function(item));
    }
    result
}
