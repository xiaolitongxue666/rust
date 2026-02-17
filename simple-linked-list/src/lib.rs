//! 单向链表：头插法、Box 实现递归结构
//!
//! 考点：Option<Box<Node>>、take() 转移所有权、FromIterator、Into<Vec>、rev 反转

use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    // 你可能想知道为什么需要 is_empty()
    // 当它可以通过 len() 轻松确定时。
    // 同时提供两者是良好的习惯，因为对于某些类型，len() 可能很昂贵，
    // 而 is_empty() 几乎总是很便宜。
    // （也问问自己，对于 SimpleLinkedList，len() 是否昂贵）
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, _element: T) {
        self.len += 1;
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        let mut cur_node = self.head;
        while let Some(node) = cur_node {
            rev_list.push(node.data);
            cur_node = node.next;
        }
        rev_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    /// 从迭代器创建链表
    ///
    /// 迭代器中的每个元素会被 push 到链表中
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in iter {
            list.push(item);
        }
        list
    }
}

// 一般来说，为 SimpleLinkedList<T> 实现 IntoIterator
// 比实现显式转换为向量更可取。这是因为，
// FromIterator 和 IntoIterator 一起使得任意集合之间的转换成为可能。
// 有了该实现，转换为向量就很简单了：
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// 本练习的 API 包含显式转换为 Vec<T> 而不是 IntoIterator 的原因是
// 实现该接口相当复杂，并且对学生的要求超出了我们在本阶段课程的预期。

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    /// 将链表转换为向量
    ///
    /// 按照链表中元素的顺序（从头部到尾部）转换为 Vec
    fn into(self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut current = self.head;

        // 遍历链表，将元素添加到向量中
        while let Some(node) = current {
            vec.push(node.data);
            current = node.next;
        }

        // 由于链表是栈式的（LIFO），需要反转向量以匹配 push 的原始顺序
        vec.reverse();
        vec
    }
}
