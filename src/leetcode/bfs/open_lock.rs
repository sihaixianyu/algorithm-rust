use std::collections::{HashSet, VecDeque};

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut res = 0;
    let deadends: HashSet<String> = deadends.into_iter().collect();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue = VecDeque::from(["0000".to_string()]);

    while queue.len() > 0 {
        let size = queue.len();
        for _ in 0..size {
            let curr = queue.pop_front().unwrap();
            if curr == target {
                return res;
            }

            if deadends.contains(&curr) {
                continue;
            }

            for i in 0..target.len() {
                let up = add_one(&curr, i);
                if !visited.contains(&up) {
                    queue.push_back(up.clone());
                    visited.insert(up);
                }

                let down = subtract_one(&curr, i);
                if !visited.contains(&down) {
                    queue.push_back(down.clone());
                    visited.insert(down);
                }
            }
        }
        res += 1;
    }

    -1
}

fn add_one(s: &String, i: usize) -> String {
    let mut up = s.clone();
    unsafe {
        let up_bytes = up.as_bytes_mut();
        if up_bytes[i] == '9' as u8 {
            up_bytes[i] = '0' as u8;
        } else {
            up_bytes[i] += 1;
        }
    }

    up
}

fn subtract_one(s: &String, i: usize) -> String {
    let mut down = s.clone();
    unsafe {
        let up_bytes = down.as_bytes_mut();
        if up_bytes[i] == '0' as u8 {
            up_bytes[i] = '9' as u8;
        } else {
            up_bytes[i] -= 1;
        }
    }

    down
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_case1() {
        let deadends = vec!["0201", "0101", "0102", "1212", "2002"];
        let deadends = deadends.into_iter().map(|str| str.to_string()).collect();
        let target = "0202".to_string();

        let res = open_lock(deadends, target);
        assert_eq!(6, res);
    }

    #[test]
    pub fn test_case2() {
        let deadends = vec!["8888"];
        let deadends = deadends.into_iter().map(|str| str.to_string()).collect();
        let target = "0009".to_string();

        let res = open_lock(deadends, target);
        assert_eq!(1, res);
    }

    #[test]
    pub fn test_case3() {
        let deadends = vec![
            "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
        ];
        let deadends = deadends.into_iter().map(|str| str.to_string()).collect();
        let target = "8888".to_string();

        let res = open_lock(deadends, target);
        assert_eq!(-1, res);
    }
}
