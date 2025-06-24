use std::{cell::RefCell, rc::Rc};

// 实现 BST 的中序遍历迭代器
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl From<TreeNode> for Option<Rc<RefCell<TreeNode>>> {
    fn from(val: TreeNode) -> Self {
        Some(Rc::new(RefCell::new(val)))
    }
}
pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            stack: if root.is_some() {
                let mut left = root;
                let mut stack = Vec::new();
                while let Some(node) = left {
                    left = node.borrow().left.clone();
                    stack.push(node);
                }
                stack
            } else {
                vec![]
            },
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> i32 {
        let curr = self.stack.pop().unwrap();
        let val = curr.borrow().val;

        if let Some(right) = curr.borrow().right.clone() {
            let mut left = right.borrow().left.clone();
            self.stack.push(right);
            while let Some(node) = left {
                left = node.borrow().left.clone();
                self.stack.push(node);
            }
        }
        val
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::{BSTIterator, TreeNode};
    #[test]
    fn test_example() {
        let root = TreeNode {
            val: 7,
            left: TreeNode {
                val: 3,
                left: None,
                right: None,
            }
            .into(),
            right: TreeNode {
                val: 15,
                left: TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }
                .into(),
                right: TreeNode {
                    val: 20,
                    left: None,
                    right: None,
                }
                .into(),
            }
            .into(),
        };

        let mut iter = BSTIterator::new(root.into());
        assert_eq!(iter.next(), 3);
        assert_eq!(iter.next(), 7);
        assert_eq!(iter.next(), 9);
        assert_eq!(iter.next(), 15);
        assert_eq!(iter.next(), 20);
        assert!(!iter.has_next());
    }
}
