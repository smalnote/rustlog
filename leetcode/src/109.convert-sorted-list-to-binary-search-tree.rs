pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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

/// 把有序的链表转换成平衡二叉树
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = Vec::with_capacity(2 * 1_0000);
        let mut node = head;
        while let Some(n) = node {
            nodes.push(n.val);
            node = n.next;
        }
        Self::sub_tree(&nodes)
    }

    fn sub_tree(nodes: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nodes.is_empty() {
            None
        } else {
            let mid = nodes.len() / 2;
            let root = TreeNode {
                val: nodes[mid],
                left: Self::sub_tree(&nodes[..mid]),
                right: Self::sub_tree(&nodes[mid + 1..]),
            };
            Some(Rc::new(RefCell::new(root)))
        }
    }
}
