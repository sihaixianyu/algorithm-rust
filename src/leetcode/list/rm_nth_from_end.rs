use crate::structure::linked_list::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let dummy = ListNode{val: 0, next: head };

    let (mut fast, mut slow) = (&dummy, &dummy);

    for _ in 0..n {
        fast= fast.next.as_ref()?;
    }

    while let Some(node) = &fast.next {
        slow = slow.next.as_ref()?;
        fast = node;
    }

    #[allow(mutable_transmutes)]
    let slow: &mut ListNode = unsafe { std::mem::transmute(slow) };
    slow.next = slow.next.take()?.next;

    dummy.next
}

#[cfg(test)]
mod tests {
    use crate::structure::linked_list::LinkedList;

    use super::*;

    #[test]
    fn test_case1() {
        let head = LinkedList::from([1, 2, 3, 4, 5].as_slice()).head;
        let n = 2;

        let ans = remove_nth_from_end(head, n);
        assert_eq!(ans, LinkedList::from([1, 2, 3, 5].as_slice()).head);
    }
}
