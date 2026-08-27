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

    #[allow(unused)]
    fn insert_tail(&mut self, val: i32) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(ListNode::new(val)));
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_debug() {
        let mut head = ListNode::new(1);
        head.insert_tail(2);
        head.insert_tail(3);
        println!("{:?}", head);
    }

    #[test]
    fn test_reverse_list() {
        let mut head = ListNode::new(1);
        head.insert_tail(2);
        head.insert_tail(3);
        let reversed = reverse_list(Some(Box::new(head)));

        let mut expected = ListNode::new(3);
        expected.insert_tail(2);
        expected.insert_tail(1);
        assert_eq!(reversed, Some(Box::new(expected)));
    }
}
