use std::cell::RefCell;
use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

/// 在一个入口为 root 的二叉树表示的房子中,节点的数字代表价值;偷任意相连的两个房
/// 子会触发报警,问如何在不触发报警的前提下,偷到最多的价值;
///
/// 记 dpy[node] 遍历以 node 为根的树的所有房子后,且偷 node ,能偷的最大总价值;
/// dpn[node] 为不偷 node 能偷的最大总价值;
/// dpy[node] = dpn[node.left] + dpn[node.right]
/// dpn[node] = max(dpy[node.left], dpn[node.left]) + max(dpy[node.right], dpy[node.right])
/// 即父节点不偷,则子节点可偷可不偷
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut dpy = HashMap::new();
        let mut dpn = HashMap::new();
        max(
            Self::roby(&mut dpy, &mut dpn, root.clone()),
            Self::robn(&mut dpy, &mut dpn, root),
        )
    }

    fn roby(
        dpy: &mut HashMap<*const TreeNode, i32>,
        dpn: &mut HashMap<*const TreeNode, i32>,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let key = node.as_ptr() as *const TreeNode;
                if let Some(val) = dpy.get(&key) {
                    return *val;
                }
                let val = Self::robn(dpy, dpn, node.borrow().left.clone())
                    + Self::robn(dpy, dpn, node.borrow().right.clone())
                    + node.borrow().val;
                dpy.insert(key, val);
                val
            }
        }
    }

    fn robn(
        dpy: &mut HashMap<*const TreeNode, i32>,
        dpn: &mut HashMap<*const TreeNode, i32>,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let key = node.as_ptr() as *const TreeNode;
                if let Some(val) = dpn.get(&key) {
                    return *val;
                }
                let val = max(
                    Self::roby(dpy, dpn, node.borrow().left.clone()),
                    Self::robn(dpy, dpn, node.borrow().left.clone()),
                ) + max(
                    Self::roby(dpy, dpn, node.borrow().right.clone()),
                    Self::robn(dpy, dpn, node.borrow().right.clone()),
                );
                dpn.insert(key, val);
                val
            }
        }
    }
}
