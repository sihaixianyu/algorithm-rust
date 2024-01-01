// This implementation of `Link` <-> `Option<Box<ListNode>>` is mainly for LeetCode test case.
// If we have to hold multiple mutable references in the same lifetime, `Option<Rc<RefCell<ListNode>>>` should be considered.

type Link = Option<Box<ListNode>>;

#[derive(Debug, PartialEq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Link,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[derive(Debug, PartialEq)]
pub struct LinkedList {
    pub head: Link,
    pub len: usize,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    pub fn traverse(&self) -> Vec<i32> {
        let mut ans = vec![];
        let mut cur = &self.head;

        while let Some(node) = cur {
            ans.push(node.val);
            cur = &node.next;
        }

        ans
    }

    pub fn index(&self, idx: usize) -> Option<i32> {
        if idx >= self.len {
            return None;
        }

        let mut cur = self.head.as_ref().unwrap();

        for _ in 0..idx {
            cur = cur.next.as_ref().unwrap()
        }

        Some(cur.val)
    }

    pub fn push_front(&mut self, val: i32) {
        let mut new_node = Box::new(ListNode::new(val));

        let Some(head) = self.head.take() else {
            self.head.as_mut().unwrap().next = Some(new_node);
            return;
        };

        new_node.next = Some(head);
        self.head = Some(new_node);
    }

    pub fn push_back(&mut self, val: i32) {
        let new_node = Box::new(ListNode::new(val));

        let Some(mut cur) = self.head.as_mut() else {
            self.head = Some(new_node);
            return;
        };

        while cur.next.is_some() {
            cur = cur.next.as_mut().unwrap()
        }

        cur.next = Some(new_node);
    }
}

impl From<&[i32]> for LinkedList {
    fn from(nums: &[i32]) -> Self {
        if nums.is_empty() {
            return LinkedList::new();
        }

        let mut head = ListNode::new(nums[0]);
        let mut cur = &mut head;

        for &v in nums.into_iter().skip(1) {
            let next = ListNode::new(v);
            cur.next = Some(Box::new(next));
            cur = cur.next.as_mut().unwrap();
        }

        LinkedList {
            head: Some(Box::new(head)),
            len: nums.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let list = LinkedList::from([1, 2, 3].as_slice());
        assert_eq!(list.traverse(), vec![1, 2, 3]);
    }

    #[test]
    fn test_index() {
        let list = LinkedList::from([1, 2, 3].as_slice());

        let Some(ans) = list.index(1) else {
            panic!("invalid index for list");
        };
        assert_eq!(ans, 2);
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::from([1, 2, 3].as_slice());

        list.push_front(0);
        assert_eq!(list.traverse(), [0, 1, 2, 3].as_slice());
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::from([1, 2, 3].as_slice());

        list.push_back(4);
        assert_eq!(list.traverse(), [1, 2, 3, 4].as_slice());
    }
}
