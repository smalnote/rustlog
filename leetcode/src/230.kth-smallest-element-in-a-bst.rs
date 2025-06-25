pub struct Solution;

// 查找 BST 中 第 k(1-indexed) 小的数
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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::dfs(root, k, 0).unwrap_err()
    }

    // 找到第 k 时返回 Err(val), 否则返回 Ok(count)
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, k: i32, p: i32) -> Result<i32, i32> {
        match node {
            None => Ok(p),
            Some(node) => {
                let left_count = Self::dfs(node.borrow().left.clone(), k, p)? + 1;
                if left_count == k {
                    return Err(node.borrow().val);
                }
                Ok(Self::dfs(node.borrow().right.clone(), k, left_count)?)
            }
        }
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
        assert_eq!(
            Solution::kth_smallest(
                TreeNode {
                    val: 5,
                    left: TreeNode {
                        val: 3,
                        left: TreeNode {
                            val: 2,
                            left: TreeNode {
                                val: 1,
                                left: None,
                                right: None
                            }
                            .into(),
                            right: None
                        }
                        .into(),
                        right: TreeNode {
                            val: 4,
                            left: None,
                            right: None
                        }
                        .into()
                    }
                    .into(),
                    right: TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }
                    .into(),
                }
                .into(),
                3
            ),
            3
        );
    }
}
