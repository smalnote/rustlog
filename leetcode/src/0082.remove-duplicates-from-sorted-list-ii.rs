pub struct Solution;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;

        while let Some(mut node) = prev.next.take() {
            let mut dup = false;
            // 检查后续是否有相同值
            while let Some(ref next) = node.next {
                if next.val == node.val {
                    // 有重复，跳过
                    node.next = node.next.take().unwrap().next;
                    dup = true;
                } else {
                    break;
                }
            }

            if dup {
                // 整段重复，丢弃 node
                prev.next = node.next;
            } else {
                // 没有重复，保留 node
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
