#[allow(non_upper_case_globals)]
pub const null: i32 = i32::MAX;

type Link = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Link,
}

impl ListNode {
    #[allow(unused)]
    pub fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None,
        }
    }

    fn as_box(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug)]
pub struct LinkedList {
    pub head: Link,
    len: usize,
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn traverse(&self) -> Vec<i32> {
        let mut res = vec![];
        let mut curr = &self.head;

        while let Some(node) = &curr {
            res.push(node.val);
            curr = &node.next;
        }

        res
    }

    pub fn from_slice(vals: &[i32]) -> LinkedList {
        if vals.len() == 0 {
            return LinkedList::new();
        }

        let mut head = ListNode::new(vals[0]).as_box();
        let mut len = 1;

        let mut curr = &mut head;
        let mut new_node: Box<ListNode>;
        for val in &vals[1..] {
            new_node = ListNode::new(*val).as_box();
            curr.next = Some(new_node);
            curr = curr.next.as_mut().unwrap();
            len += 1;
        }

        LinkedList {
            head: Some(head),
            len: len,
        }
    }
}

pub fn traverse(head: &Link) -> Vec<i32> {
    let mut res = vec![];
    let mut curr = head;

    while let Some(node) = &curr {
        res.push(node.val);
        curr = &node.next;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_slice() {
        let list = LinkedList::from_slice(&vec![1, 2, 3, 4, 5][..]);

        assert_eq!(vec![1, 2, 3, 4, 5], list.traverse());
    }

    #[test]
    fn test_traverse() {
        let list = LinkedList::from_slice(&vec![1, 2, 3, 4, 5][..]);

        assert_eq!(vec![1, 2, 3, 4, 5], traverse(&list.head));
    }
}
