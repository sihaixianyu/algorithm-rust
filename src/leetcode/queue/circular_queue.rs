pub struct MyCircularQueue {
    nums: Vec<i32>,
    head: usize,
    tail: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        MyCircularQueue {
            nums: vec![0; (k+1) as usize],
            head: 0,
            tail: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        self.nums[self.tail] = value;
        self.tail = (self.tail + 1) % self.nums.len();

        true
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.head = (self.head + 1) % self.nums.len();

        true
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            return -1
        }
        
        self.nums[self.head]
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1
        }

        self.nums[(self.tail + self.nums.len() - 1) % self.nums.len()]
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        (self.tail + 1) % self.nums.len() == self.head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut circular_queue = MyCircularQueue::new(3);
        circular_queue.en_queue(1);
        circular_queue.en_queue(2);
        circular_queue.en_queue(3);
        circular_queue.en_queue(4);

        assert_eq!(circular_queue.rear(), 3);
        assert_eq!(circular_queue.is_full(), true);
        assert_eq!(circular_queue.de_queue(), true);
        assert_eq!(circular_queue.en_queue(4), true);
        assert_eq!(circular_queue.rear(), 4);
    }
}
