use crate::structure::linked_list::ListNode;

// 反转链表：https://leetcode.cn/problems/reverse-linked-list/
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut pre = None;
    let mut cur = head;

    while let Some(mut node) = cur.take() {
        let next = node.next;
        node.next = pre;
        pre = Some(node);
        cur = next;
    }

    pre
}

pub fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn helper(pre: Option<Box<ListNode>>, mut cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = cur.take() {
            let next = node.next;
            node.next = pre;
            // The first param should have been `curr`, but its inner value was moved, we have to make a new option.
            return helper(Some(node), next);
        }
        pre
    }

    helper(None, head)
}

#[cfg(test)]
mod tests {
    use super::{reverse_list, reverse_list2};
    use crate::structure::linked_list::LinkedList;

    #[test]
    fn test_case1() {
        let list = LinkedList::from([1, 2, 3, 4, 5].as_slice());

        let mut ans = LinkedList::new();
        ans.head = reverse_list(list.head);
        ans.len = list.len;
        assert_eq!(ans.traverse(), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_case2() {
        let list = LinkedList::from([1, 2, 3, 4, 5].as_slice());

        let mut ans = LinkedList::new();
        ans.head = reverse_list2(list.head);
        ans.len = list.len;
        assert_eq!(ans.traverse(), vec![5, 4, 3, 2, 1]);
    }
}
