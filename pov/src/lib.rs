//! # POV (Point of View) - 树重定根
//!
//! 将树以指定节点为根重新定向。树是无环连通图，任意两点间有唯一路径。
//!
//! ## 考点
//! - 递归：在子节点中查找目标，交换根与目标
//! - `binary_search_by` + `unwrap_err` 保持子节点有序（相等性不依赖顺序）
//! - `mem::swap` 交换根与子节点

use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree<T: Debug + Ord> {
    label: T,
    children: Vec<Box<Tree<T>>>,
}

impl<T: Debug + Ord> Tree<T> {
    pub fn new(label: T) -> Self {
        Self {
            label,
            children: Vec::new(),
        }
    }

    /// 添加子节点，按 label 有序插入以保证相等性不依赖子节点顺序
    pub fn with_child(mut self, child: Self) -> Self {
        let pos = self
            .children
            .binary_search_by(|c| c.label.cmp(&child.label))
            .unwrap_err();
        self.children.insert(pos, Box::new(child));
        self
    }

    /// 以 from 为根重定树，返回是否找到该节点
    pub fn pov_from(&mut self, from: &T) -> bool {
        self.pov_from_rec(from).is_some()
    }

    /// 递归：找到 from 则返回路径索引，并就地交换根与 from
    fn pov_from_rec(&mut self, from: &T) -> Option<Vec<usize>> {
        if &self.label == from {
            return Some(Vec::new());
        }

        let (pos, mut index_list) = self
            .children
            .iter_mut()
            .enumerate()
            .find_map(|(i, child)| child.pov_from_rec(from).map(|il| (i, il)))?;

        let mut old_pov = self.children.remove(pos);
        std::mem::swap(self, &mut old_pov);

        let mut parent_of_old_pov = self;
        for i in index_list.iter().rev() {
            parent_of_old_pov = &mut parent_of_old_pov.children[*i];
        }

        let new_idx = parent_of_old_pov
            .children
            .binary_search_by(|c| c.label.cmp(&old_pov.label))
            .unwrap_err();
        parent_of_old_pov.children.insert(new_idx, old_pov);
        index_list.push(new_idx);

        Some(index_list)
    }

    /// 返回 from 到 to 的路径：先重定根到 to，再找 from 到根的路径
    pub fn path_between<'a>(&'a mut self, from: &'a T, to: &'a T) -> Option<Vec<&'a T>> {
        if !self.pov_from(to) {
            return None;
        }
        self.path_from(from)
    }

    fn path_from<'a>(&'a self, from: &'a T) -> Option<Vec<&'a T>> {
        if &self.label == from {
            return Some(vec![from]);
        }
        let mut path = self.children.iter().find_map(|c| c.path_from(from))?;
        path.push(&self.label);
        Some(path)
    }
}
