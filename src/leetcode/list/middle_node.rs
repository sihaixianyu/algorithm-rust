use crate::structure::linked_list::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut slow, mut fast) = (&head, &head);

    while fast.as_ref().is_some() && fast.as_ref()?.next.is_some() {
        slow = &slow.as_ref()?.next;
        fast = &fast.as_ref()?.next.as_ref()?.next;
    }

    slow.clone()
}

#[cfg(test)]
mod tests {
    use crate::structure::linked_list::LinkedList;

    use super::*;

    #[test]
    fn test_case1() {
        let head = LinkedList::from_slice(&vec![1, 2, 3, 4, 5]).head;

        let res = middle_node(head);
        assert_eq!(3, res.unwrap().val);
    }
}
