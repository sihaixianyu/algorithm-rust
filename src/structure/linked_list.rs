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
    fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None,
        }
        
    }
}
