use crate::structure::linked_list::ListNode;

// 两两交换链表中的节点：https://leetcode.cn/problems/swap-nodes-in-pairs/
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;

    let mut pre = &mut dummy;

    while let Some(mut first_node) = pre.next.take() {
        if let Some(mut second_node) = first_node.next.take() {
            first_node.next = second_node.next.take();
            second_node.next = Some(first_node);

            pre.next = Some(second_node);
            pre = pre.next.as_mut().unwrap().next.as_mut().unwrap(); // Can't use `first_node` because it's a temporary borrow.
        } else {
            // Used for one-node-left situation, we must reconnect the `first_node` taken before to `pre`.
            pre.next = Some(first_node);
            break;
        }
    }

    dummy.next
}

pub fn swap_pairs2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn helper(first: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if first == None || first.as_ref().unwrap().next == None {
            return first;
        }

        let mut first_node = first.unwrap();

        if let Some(mut second_node) = first_node.next.take() {
            first_node.next = helper(second_node.next);
            second_node.next = Some(first_node);
            Some(second_node)
        } else {
            Some(first_node)
        }
    }

    helper(head)
}

#[cfg(test)]
mod tests {
    use crate::structure::linked_list::LinkedList;

    use super::*;

    #[test]
    fn test_swap_pairs() {
        let list = LinkedList::from([1, 2, 3, 4].as_slice());

        let ans = swap_pairs(list.head);
        assert_eq!(ans, LinkedList::from([2, 1, 4, 3].as_slice()).head);
    }

    #[test]
    fn test_swap_pairs2() {
        let list = LinkedList::from([1, 2, 3, 4].as_slice());

        let ans = swap_pairs2(list.head);
        assert_eq!(ans, LinkedList::from([2, 1, 4, 3].as_slice()).head);
    }
}
