pub struct Solution;

// 反转完全二叉树的奇数层级，根是第 0 层，往下 +1
// 用 BFS
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
use std::collections::VecDeque;
use std::mem::swap;
use std::rc::Rc;
impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let mut q = VecDeque::<Rc<RefCell<TreeNode>>>::new();
        q.push_back(root.clone().unwrap());

        let mut level = 0;
        while q[0].borrow().left.is_some() {
            level += 1;
            let len = q.len();
            for _ in 0..len {
                let curr = q.pop_front().unwrap();
                q.push_back(curr.borrow().left.clone().unwrap());
                q.push_back(curr.borrow().right.clone().unwrap());
            }
            if level & 1 == 1 {
                let (mut i, mut j) = (0, q.len() - 1);
                while i < j {
                    swap(&mut q[i].borrow_mut().val, &mut q[j].borrow_mut().val);
                    (i, j) = (i + 1, j - 1);
                }
            }
        }

        root
    }
}

impl From<TreeNode> for Option<Rc<RefCell<TreeNode>>> {
    fn from(val: TreeNode) -> Self {
        Some(Rc::new(RefCell::new(val)))
    }
}

#[cfg(test)]
mod test {
    use super::{Solution, TreeNode};
    #[test]
    fn test_example() {
        assert!(Solution::reverse_odd_levels(
            TreeNode {
                val: 7,
                left: TreeNode {
                    val: 13,
                    left: None,
                    right: None
                }
                .into(),
                right: TreeNode {
                    val: 11,
                    left: None,
                    right: None
                }
                .into(),
            }
            .into()
        )
        .is_some());
    }
}
