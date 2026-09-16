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

pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k == 0 {
        return head;
    }

    // 1. 获取链表长度
    let mut length = 1;
    let mut tail = head.as_ref().unwrap();

    while let Some(next) = tail.next.as_ref() {
        length += 1;
        tail = next;
    }

    let k = k % length;

    if k == 0 {
        return head;
    }

    // 2. 找到新的尾节点
    let new_tail_index = length - k - 1;

    let mut new_tail = head.as_mut().unwrap();

    for _ in 0..new_tail_index {
        new_tail = new_tail.next.as_mut().unwrap();
    }

    // 3. new_tail.next 就是新的头节点
    let mut new_head = new_tail.next.take();

    // 4. 找到原来的尾节点
    let mut old_tail = new_head.as_mut().unwrap();

    while old_tail.next.is_some() {
        old_tail = old_tail.next.as_mut().unwrap();
    }

    // 5. 原尾节点连接到原来的 head
    old_tail.next = head;

    // 6. 返回新的头
    new_head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        // head: 1-2-3-4-5, k = 2
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

        let result = rotate_right(head, 2);
        let mut target = Some(Box::new(ListNode::new(4)));
        target.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
        target.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
        target
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(2)));
        target
            .as_mut()
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
            .next = Some(Box::new(ListNode::new(3)));

        assert_eq!(result, target);
    }
}
