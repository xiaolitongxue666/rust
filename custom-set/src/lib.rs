/// 自定义集合类型，使用 Vec 存储元素
/// 集合中的元素唯一，顺序不影响相等性判断
#[derive(Debug)]
pub struct CustomSet<T> {
    /// 存储集合元素的 Vec
    elements: Vec<T>,
}

impl<T: Eq + Clone> CustomSet<T> {
    /// 从切片创建新的集合，自动去除重复元素
    pub fn new(input: &[T]) -> Self {
        let mut elements = Vec::new();
        for item in input {
            if !elements.contains(item) {
                elements.push(item.clone());
            }
        }
        Self { elements }
    }

    /// 检查集合是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// 检查集合是否包含指定元素
    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    /// 向集合中添加元素，如果元素已存在则不添加
    pub fn add(&mut self, element: T) {
        if !self.elements.contains(&element) {
            self.elements.push(element);
        }
    }

    /// 检查当前集合是否为另一个集合的子集
    /// 空集是任何集合的子集
    pub fn is_subset(&self, other: &Self) -> bool {
        // 如果当前集合为空，则它是任何集合的子集
        if self.is_empty() {
            return true;
        }
        // 检查当前集合的每个元素是否都在另一个集合中
        self.elements.iter().all(|elem| other.contains(elem))
    }

    /// 检查两个集合是否不相交（没有共同元素）
    /// 空集与任何集合都不相交
    pub fn is_disjoint(&self, other: &Self) -> bool {
        // 如果任一集合为空，则它们不相交
        if self.is_empty() || other.is_empty() {
            return true;
        }
        // 检查是否有共同元素
        !self.elements.iter().any(|elem| other.contains(elem))
    }

    /// 计算两个集合的交集（共同元素）
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for elem in &self.elements {
            if other.contains(elem) && !result.contains(elem) {
                result.push(elem.clone());
            }
        }
        Self { elements: result }
    }

    /// 计算两个集合的差集（在第一个集合中但不在第二个集合中的元素）
    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        for elem in &self.elements {
            if !other.contains(elem) && !result.contains(elem) {
                result.push(elem.clone());
            }
        }
        Self { elements: result }
    }

    /// 计算两个集合的并集（所有唯一元素）
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut result = Vec::new();
        // 添加第一个集合的所有元素
        for elem in &self.elements {
            if !result.contains(elem) {
                result.push(elem.clone());
            }
        }
        // 添加第二个集合中不在第一个集合中的元素
        for elem in &other.elements {
            if !result.contains(elem) {
                result.push(elem.clone());
            }
        }
        Self { elements: result }
    }
}

/// 实现 PartialEq，集合相等不依赖于元素顺序
impl<T: Eq + Clone> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        // 如果大小不同，则不等
        if self.elements.len() != other.elements.len() {
            return false;
        }
        // 检查当前集合的每个元素是否都在另一个集合中
        self.elements.iter().all(|elem| other.contains(elem))
    }
}

/// 实现 Eq trait
impl<T: Eq + Clone> Eq for CustomSet<T> {}
