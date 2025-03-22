use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

struct Node {
    key: i32,
    val: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            val: value,
            prev: None,
            next: None,
        }
    }
}

pub struct LRUCache {
    capacity: i32,
    dummy: Rc<RefCell<Node>>,
    key_to_node: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let dummy = Rc::new(RefCell::new(Node::new(0, 0)));
        let key_to_node = HashMap::new();

        dummy.borrow_mut().next = Some(dummy.clone());
        dummy.borrow_mut().prev = Some(Rc::downgrade(&dummy));

        LRUCache {
            capacity,
            dummy,
            key_to_node,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.key_to_node.get(&key) {
            let node = node.clone();

            self.remove_by_node(node.clone());
            self.push_front(node.clone());

            return node.borrow().val;
        }

        -1
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.key_to_node.get(&key) {
            node.borrow_mut().val = value;
            self.get(key);
            return;
        }

        if self.key_to_node.len() == self.capacity as usize {
            let last = self
                .dummy
                .borrow_mut()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap();
            self.key_to_node.remove(&last.borrow().key);
            self.remove_by_node(last);
        }

        let node = Rc::new(RefCell::new(Node {
            key,
            val: value,
            prev: None,
            next: None,
        }));
        self.key_to_node.insert(key, node.clone());
        self.push_front(node);
    }

    fn remove_by_node(&mut self, node: Rc<RefCell<Node>>) {
        let prev = node.borrow_mut().prev.take().unwrap().upgrade().unwrap();
        let next = node.borrow_mut().next.take().unwrap();

        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
    }

    fn push_front(&mut self, node: Rc<RefCell<Node>>) {
        let next = self.dummy.borrow_mut().next.take().unwrap();

        self.dummy.borrow_mut().next = Some(node.clone());
        next.borrow_mut().prev = Some(Rc::downgrade(&node));

        node.borrow_mut().next = Some(next.clone());
        node.borrow_mut().prev = Some(Rc::downgrade(&self.dummy));
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn test_case_1() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
