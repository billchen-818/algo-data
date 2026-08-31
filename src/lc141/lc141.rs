// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while let (Some(s), Some(f)) = (slow, fast) {
        slow = s.next.as_ref();
        fast = f.next.as_ref().and_then(|n| n.next.as_ref());

        if slow == fast {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cycle() {
        let mut head = ListNode::new(1);
        let mut second = ListNode::new(2);
        let mut third = ListNode::new(3);
        head.next = Some(Box::new(second));
        head.next.as_mut().unwrap().next = Some(Box::new(third));
        // Create a cycle for testing
        head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(head.clone()));
        assert_eq!(has_cycle(Some(Box::new(head))), true);
    }
}
