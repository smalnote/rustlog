pub struct Solution;

// 二叉树的根到叶子节点的路径代表一个十进制数，求所有数的和，每个节点的数为 0-9
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_node(root, 0)
    }
    fn sum_node(node: Option<Rc<RefCell<TreeNode>>>, base: i32) -> i32 {
        let n = node.unwrap();
        let (left, right) = (n.borrow().left.clone(), n.borrow().right.clone());
        let base = base * 10 + n.borrow().val;
        match (left, right) {
            (None, None) => base,
            (Some(left), Some(right)) => {
                Self::sum_node(Some(left), base) + Self::sum_node(Some(right), base)
            }
            (Some(left), None) => Self::sum_node(Some(left), base),
            (None, Some(right)) => Self::sum_node(Some(right), base),
        }
    }
}
