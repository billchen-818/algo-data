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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut fast = dummy.clone();

    // 快指针先走N步
    for _ in 0..n {
        if let Some(next) = fast.next.as_mut() {
            fast = next.clone();
        } else {
            return None;
        }
    }

    // 快慢指针一起走，快指针走到末尾慢指针刚好走到倒数第N个节点的前一个节点
    let mut slow = dummy.as_mut();

    while fast.next.is_some() {
        fast = fast.next.as_mut().unwrap().clone();
        slow = slow.next.as_mut().unwrap();
    }

    slow.next = slow.next.take().and_then(|node| node.next);
    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        let head = Some(Box::new(ListNode::new(1)));
        let result = remove_nth_from_end(head, 1);

        assert_eq!(result, None);

        // head = [1,2,3,4,5], n = 2
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(4)));
        head.as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(5)));
        let result = remove_nth_from_end(head, 2);
        let mut expected = Some(Box::new(ListNode::new(1)));
        expected.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        expected.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        expected
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(5)));
        assert_eq!(result, expected);
    }
}
