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

#[cfg(test)]
mod tests {
    use crate::structure::linked_list::{self, LinkedList};

    use super::*;

    #[test]
    fn test_case1() {
        let list1 = LinkedList::from_slice(&[1, 4, 5]);
        // dbg!(&list1);
        let list2 = LinkedList::from_slice(&[1, 3, 4]);
        // dbg!(&list2);
        let list3 = LinkedList::from_slice(&[2, 6]);
        // dbg!(&list3);
        let lists = vec![list1.head, list2.head, list3.head];

        let res = merge_k_lists(lists);
        assert_eq!(vec![1, 1, 2, 3, 4, 4, 5, 6], linked_list::traverse(&res));
    }
}
