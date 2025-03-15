use crate::structure::linked_list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut node) = curr.take() {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        curr = next;
    }

    prev
}

pub fn reverse_list_pro(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    unsafe fn reverse(curr: *mut ListNode) -> (*mut ListNode, *mut ListNode) {
        if (*curr).next.is_none() {
            return (curr, curr);
        }

        let raw_next = Box::into_raw((*curr).next.take().unwrap());
        let (next, tail) = reverse(raw_next);
        (*next).next = Some(Box::from_raw(curr));

        return (curr, tail);
    }

    let Some(curr) = head else {
        return None;
    };

    unsafe {
        let (_, tail) = reverse(Box::into_raw(curr));
        Some(Box::from_raw(tail))
    }
}

#[cfg(test)]
mod tests {
    use super::{reverse_list, reverse_list_pro};
    use crate::structure::linked_list::LinkedList;

    #[test]
    fn test_case_0() {
        let list = LinkedList::from([1, 2, 3, 4, 5].as_slice());

        let mut ans = LinkedList::new();
        ans.head = reverse_list(list.head);
        ans.len = list.len;
        assert_eq!(ans.traverse(), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_case_1() {
        let list = LinkedList::from([1, 2, 3, 4, 5].as_slice());

        let mut ans = LinkedList::new();
        ans.head = reverse_list_pro(list.head);
        ans.len = list.len;
        assert_eq!(ans.traverse(), vec![5, 4, 3, 2, 1]);
    }
}
