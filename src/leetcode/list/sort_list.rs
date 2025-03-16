use crate::structure::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() {
            return head;
        }

        let mut next_head = Solution::find_mid(&head);
        head = Solution::sort_list(head);
        next_head = Solution::sort_list(next_head);

        Solution::merge_list(head, next_head)
    }

    fn find_mid(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (head, head);

        while fast.is_some() && fast.as_ref()?.next.is_some() {
            slow = &slow.as_ref()?.next;
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }

        #[allow(mutable_transmutes)]
        let next_head: &mut Option<Box<ListNode>> = unsafe { std::mem::transmute(slow) };

        next_head.take()
    }

    fn merge_list(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut curr = &mut dummy;

        while let (Some(n1), Some(n2)) = (&l1, &l2) {
            if n1.val < n2.val {
                curr.next = l1.take();
                curr = curr.next.as_mut()?;
                l1 = curr.next.take();
            } else {
                curr.next = l2.take();
                curr = curr.next.as_mut()?;
                l2 = curr.next.take();
            }
        }

        curr.next = l1.or(l2);

        dummy.next.take()
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::linked_list::{traverse, LinkedList};

    use super::*;

    #[test]
    fn test_case_0() {
        let list = LinkedList::from([4, 2, 1, 3].as_slice());

        let sorted_head = Solution::sort_list(list.head);
        let sorted_nums = traverse(&sorted_head);

        assert_eq!(sorted_nums, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_case_1() {
        let list = LinkedList::from([-1, 5, 3, 4, 0].as_slice());

        let sorted_head = Solution::sort_list(list.head);
        let sorted_nums = traverse(&sorted_head);

        assert_eq!(sorted_nums, vec![-1, 0, 3, 4, 5]);
    }
}
