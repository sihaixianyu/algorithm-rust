use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;

use crate::structure::linked_list::ListNode;

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut heap = BinaryHeap::new();

        for b in lists {
            if let Some(n) = b {
                heap.push(n);
            }
        }

        let mut curr = &mut dummy;
        while let Some(mut node) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(next);
            }

            curr.next = Some(node);
            curr = curr.next.as_mut().unwrap();
        }

        dummy.next
    }
}

mod tests {
    use crate::structure::linked_list::{traverse, LinkedList};

    use super::Solution;

    #[test]
    fn test_case_1() {
        let list_1 = LinkedList::from([1, 4, 5].as_slice());
        let list_2 = LinkedList::from([1, 3, 4].as_slice());
        let list_3 = LinkedList::from([2, 6].as_slice());
        let heads = vec![list_1.head, list_2.head, list_3.head];

        let rev_head = Solution::merge_k_lists(heads);
        let res = traverse(&rev_head);

        assert_eq!(res, vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn test_case_2() {
        let heads = vec![];

        let rev_head = Solution::merge_k_lists(heads);
        let res = traverse(&rev_head);

        assert_eq!(res, vec![]);
    }
}
