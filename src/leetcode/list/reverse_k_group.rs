use crate::structure::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut next_head = &mut head;

        for _ in 0..k {
            if next_head.is_none() {
                return head;
            }
            next_head = &mut next_head.as_mut()?.next;
        }

        let mut new_next_head = Solution::reverse_k_group(next_head.take(), k);

        for _ in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                node.next = new_next_head.take();
                new_next_head = Some(node);
            }
        }

        new_next_head
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::linked_list::{traverse, LinkedList};

    use super::Solution;

    #[test]
    fn test_case_0() {
        let list = LinkedList::from([1, 2, 3, 4, 5].as_slice());
        let k = 2;

        let rev_head = Solution::reverse_k_group(list.head, k);
        let res = traverse(&rev_head);

        assert_eq!(res, vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn test_case_1() {
        let list = LinkedList::from([1, 2, 3, 4, 5].as_slice());
        let k = 3;

        let rev_head = Solution::reverse_k_group(list.head, k);
        let res = traverse(&rev_head);

        assert_eq!(res, vec![3, 2, 1, 4, 5]);
    }
}
