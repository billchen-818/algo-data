// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(mut l1), Some(mut l2)) => {
            if l1.val < l2.val {
                let next = l1.next.take();
                l1.next = merge_two_lists(next, Some(l2));
                Some(l1)
            } else {
                let next = l2.next.take();
                l2.next = merge_two_lists(Some(l1), next);
                Some(l2)
            }
        }
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        // list1 = [1, 2, 4], list2 = [1, 3, 4]
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let result = merge_two_lists(list1, list2);
        // The expected result is [1, 1, 2, 3, 4, 4]
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(4))),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(result, expected);
        assert!(result.is_some());

        // list1 = [], list2 = []
        let list1 = None;
        let list2 = None;
        let result = merge_two_lists(list1, list2);
        assert_eq!(result, None);

        // list1 = [], list2 = [0]
        let list1 = None;
        let list2 = Some(Box::new(ListNode::new(0)));
        let result = merge_two_lists(list1, list2);
        let expected = Some(Box::new(ListNode::new(0)));
        assert_eq!(result, expected);
    }
}
