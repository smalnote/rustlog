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
pub struct Solution();
// @leet start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut queue = VecDeque::new();
        if let Some(node) = root {
            queue.push_back(node);
        }

        while !queue.is_empty() {
            let len = queue.len();
            let mut max = i32::MIN;
            for _ in 0..len {
                let node = queue.pop_front().unwrap();
                if let Some(ref left) = node.borrow().left {
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = node.borrow().right {
                    queue.push_back(right.clone());
                }
                if max < node.borrow().val {
                    max = node.borrow().val
                }
            }
            result.push(max);
        }

        result
    }
}
// @leet end

