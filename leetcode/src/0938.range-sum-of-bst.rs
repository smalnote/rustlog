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
pub struct Solution();
// @leet start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => 0,
            Some(ref n) => match n.borrow().val {
                v if v < low => Solution::range_sum_bst(n.borrow().right.clone(), low, high),
                v if low <= v && v <= high => {
                    v + Solution::range_sum_bst(n.borrow().left.clone(), low, high)
                        + Solution::range_sum_bst(n.borrow().right.clone(), low, high)
                }
                _ => Solution::range_sum_bst(n.borrow().left.clone(), low, high),
            },
        }
    }
}
// @leet end

