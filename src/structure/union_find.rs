pub struct UnionFind {
    ids: Vec<i32>,
    count: i32,
}

impl UnionFind {
    pub fn new(n: i32) -> UnionFind {
        let cnt = n;
        let ids = Vec::from_iter(0..n);

        UnionFind { ids, count: cnt }
    }

    pub fn union(&mut self, p: i32, q: i32) {
        let p_root = self.find(p);
        let q_root = self.find(q);

        if p_root == q_root {
            return;
        }

        self.ids[p_root as usize] = self.ids[q_root as usize];
        self.count -= 1;
    }

    pub fn find(&mut self, mut p: i32) -> i32 {
        while self.ids[p as usize] != p {
            p = self.ids[p as usize];
        }

        p
    }

    pub fn connected(&mut self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

pub struct WeightedUnionFind {
    ids: Vec<i32>,
    sizes: Vec<i32>,
    count: i32,
}

impl WeightedUnionFind {
    pub fn new(n: i32) -> WeightedUnionFind {
        let count = n;
        let ids = Vec::from_iter(0..n);
        let sizes = vec![1; n as usize];

        WeightedUnionFind { ids, sizes, count }
    }

    pub fn union(&mut self, p: i32, q: i32) {
        let p_root = self.find(p) as usize;
        let q_root = self.find(q) as usize;

        if p_root == q_root {
            return;
        }

        // Let the lower tree join the higher tree.
        if self.sizes[p_root] < self.sizes[q_root] {
            self.ids[p_root] = self.ids[q_root];
            self.sizes[q_root] += self.sizes[p_root];
        } else {
            self.ids[q_root] = self.ids[p_root];
            self.sizes[p_root] += self.sizes[q_root];
        }

        self.count -= 1;
    }

    pub fn find(&mut self, mut p: i32) -> i32 {
        while self.ids[p as usize] != p {
            p = self.ids[p as usize];
        }

        p
    }

    pub fn connected(&mut self, p: i32, q: i32) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod union_find_tests {
        use super::*;

        #[test]
        fn test_case1() {
            let mut uf = UnionFind::new(5);
            uf.union(0, 1);
            uf.union(0, 2);
            uf.union(0, 3);
            uf.union(0, 4);

            assert!(uf.connected(0, 1));
            assert!(uf.connected(1, 2));
            assert!(uf.connected(2, 3));
            assert!(uf.connected(3, 4));
            assert_eq!(uf.count(), 1); // Only one forest left
        }
    }

    mod weighted_union_find_tests {
        use super::*;

        #[test]
        fn test_case1() {
            let mut uf = WeightedUnionFind::new(5);
            uf.union(0, 1);
            uf.union(0, 2);
            uf.union(0, 3);
            uf.union(0, 4);

            assert!(uf.connected(0, 1));
            assert!(uf.connected(1, 2));
            assert!(uf.connected(2, 3));
            assert!(uf.connected(3, 4));
            assert_eq!(uf.count(), 1); // Only one forest left
        }
    }
}
