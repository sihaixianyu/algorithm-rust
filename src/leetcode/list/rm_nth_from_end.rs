use crate::structure::linked_list::ListNode;

// 删除链表的倒数第 N 个结点：https://leetcode.cn/problems/remove-nth-node-from-end-of-list/
pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head.take();

    let mut fast = &Box::clone(&dummy);
    let mut slow = &mut dummy;

    for _ in 0..n {
        fast = fast.next.as_ref().unwrap();
    }

    while fast.next != None {
        fast = fast.next.as_ref().unwrap();
        slow = slow.next.as_mut().unwrap();
    }

    println!("{}", slow.val);
    slow.next = slow.next.as_mut().unwrap().next.take();

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
