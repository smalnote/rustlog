pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode { val: 0, next: head };

        Self::swap_pairs_helper(&mut dummy)
    }

    fn swap_pairs_helper(node: &mut ListNode) -> Option<Box<ListNode>> {
        if node.next.is_none() || node.next.as_ref()?.next.is_none() {
            return node.next.take();
        }

        let mut first = node.next.take().unwrap();
        let mut second = first.next.take().unwrap();

        first.next = Self::swap_pairs_helper(&mut second);

        second.next = Some(first);

        Some(second)
    }
}
