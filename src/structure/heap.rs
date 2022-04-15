pub struct Heap<T: Default> {
    len: usize,
    vals: Vec<T>,
    less: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(less: fn(&T, &T) -> bool) -> Self {
        Self {
            len: 0,
            vals: vec![T::default()],
            less,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, val: T) {
        self.len += 1;
        self.vals.push(val);

        // Heapify from bottom to top
        self.heapify_up(self.len)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }

        let len = self.len();
        self.vals.swap(1, len);
        self.len -= 1;
        self.heapify_down(1);

        self.vals.pop()
    }

    fn heapify_up(&mut self, mut child: usize) {
        while child > 1 && (self.less)(&self.vals[child], &self.vals[child / 2]) {
            self.vals.swap(child / 2, child);
            child /= 2;
        }
    }

    fn heapify_down(&mut self, mut parent: usize) {
        let len = self.len();
        while 2 * parent <= len {
            let mut j = 2 * parent;
            if j < len && (self.less)(&self.vals[j + 1], &self.vals[j]) {
                j += 1;
            }

            if (self.less)(&self.vals[parent], &self.vals[j]) {
                break;
            }

            self.vals.swap(parent, j);
            parent = j;
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Default + Ord>() -> Heap<T> {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Default + Ord>() -> Heap<T> {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.push(4);
        heap.push(2);
        heap.push(9);
        heap.push(11);

        assert_eq!(heap.is_empty(), false);
        assert_eq!(heap.len(), 4);

        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(11));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.push(4);
        heap.push(2);
        heap.push(9);
        heap.push(11);

        assert_eq!(heap.is_empty(), false);
        assert_eq!(heap.len(), 4);

        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), None);
    }
}
