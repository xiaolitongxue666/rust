use std::ops::Add;

pub struct Triangle<T>
where
    T: Copy + PartialEq + PartialOrd + Add<Output = T> + Default,
{
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Copy + PartialEq + PartialOrd + Add<Output = T> + Default,
{
    /// 构建三角形
    ///
    /// 验证三角形不等式：任意两边之和必须大于第三边
    /// 同时检查是否有零边
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;

        // 检查是否有零边
        if a == T::default() || b == T::default() || c == T::default() {
            return None;
        }

        // 验证三角形不等式：任意两边之和必须大于第三边
        if a + b <= c || b + c <= a || c + a <= b {
            return None;
        }

        Some(Triangle { sides })
    }

    /// 判断是否为等边三角形（三边都相等）
    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    /// 判断是否为不等边三角形（三边都不相等）
    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[1] != self.sides[2]
            && self.sides[2] != self.sides[0]
    }

    /// 判断是否为等腰三角形（至少有两边相等）
    /// 注意：等边三角形也是等腰三角形
    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[2] == self.sides[0]
    }
}
