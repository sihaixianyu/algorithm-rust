use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            val: val,
            next: None,
        }
    }
}

fn print(n: &Option<Rc<RefCell<Node>>>) {
    println!("{:?}", n);
}

fn main() {
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    assert!(s1 == s2);

    let head = Some(Rc::new(RefCell::new(Node::new(0))));
    head.as_ref().unwrap().borrow_mut().next = Some(Rc::new(RefCell::new(Node::new(0))));

    if let Some(rc) = head {
        print(&rc.borrow().next);
    }
}
