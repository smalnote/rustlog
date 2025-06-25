pub struct Solution;
// 想像你从二叉树的右边向左看，左边的节点会被同高度的右边的节点挡住，列出你可以
// 看到的右边的节点，从 root 到 left 排序
//
// 深度优选遍历，计算右边边高度和当前高度决定当前节点是否可见
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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut list = Vec::with_capacity(128);
        Self::dfs(root, 0, 0, &mut list);
        list
    }

    fn dfs(
        node: Option<Rc<RefCell<TreeNode>>>,
        mut curr_height: usize,
        mut right_height: usize,
        list: &mut Vec<i32>,
    ) -> usize {
        match node {
            Some(node) => {
                curr_height += 1;
                if curr_height > right_height {
                    right_height = curr_height;
                    list.push(node.borrow().val);
                }
                right_height =
                    Self::dfs(node.borrow().right.clone(), curr_height, right_height, list);
                Self::dfs(node.borrow().left.clone(), curr_height, right_height, list)
            }
            None => right_height,
        }
    }
}

#[cfg(test)]
mod test {

    use super::{Solution, TreeNode};
    #[test]
    fn test_example() {
        assert_eq!(
            Solution::right_side_view(
                TreeNode {
                    val: 1,
                    left: TreeNode {
                        val: 2,
                        left: TreeNode {
                            val: 4,
                            left: TreeNode {
                                val: 5,
                                left: None,
                                right: None,
                            }
                            .into(),
                            right: None,
                        }
                        .into(),
                        right: None,
                    }
                    .into(),
                    right: TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }
                    .into(),
                }
                .into()
            ),
            vec![1, 3, 4, 5]
        );
    }
}
