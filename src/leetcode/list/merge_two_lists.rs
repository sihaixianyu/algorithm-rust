use crate::structure::linked_list::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val < n2.val {
                n1.next = merge_two_lists(n1.next, Some(n2));
                Some(n1)
            } else {
                n2.next = merge_two_lists(Some(n1), n2.next);
                Some(n2)
            }
        }
    }
}

// pub fn merge_two_lists(
//     list1: Option<Box<ListNode>>,
//     list2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let mut dummy = ListNode::new(0);
//     let mut p = &mut dummy;
//     let (mut p1, mut p2) = (list1, list2);

//     while p1.is_some() && p2.is_some() {
//         let (n1, n2) = (p1.as_ref().unwrap(), p2.as_ref().unwrap());
//         if n1.val < n2.val {
//             p.next = p1;
//             p = p.next.as_mut().unwrap();
//             p1 = p.next.take();
//         } else {
//             p.next = p2;
//             p = p.next.as_mut().unwrap();
//             p2 = p.next.take();
//         }
//     }

//     if p1.is_none() {
//         p.next = p2;
//     } else {
//         p.next = p1;
//     }

//     dummy.next
// }

#[cfg(test)]
mod tests {
    use crate::structure::linked_list::{self, LinkedList};

    use super::*;

    #[test]
    fn test_case1() {
        let list1 = LinkedList::from_slice(&vec![1, 2, 4][..]);
        let list2 = LinkedList::from_slice(&vec![1, 3, 4][..]);

        let res = merge_two_lists(list1.head, list2.head);
        assert_eq!(vec![1, 1, 2, 3, 4, 4], linked_list::traverse(&res));
    }
}
