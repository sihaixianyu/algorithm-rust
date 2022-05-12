use std::collections::{HashSet, VecDeque};

pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();

    let mut window = VecDeque::new();
    let mut record = HashSet::new();
    let mut max = 0;

    let (mut lp, mut rp) = (0 as usize, 0 as usize);
    while rp < s.len() {
        let c = s[rp];
        window.push_back(c);
        rp += 1;

        if record.contains(&c) {
            while lp < rp && s[lp] != c {
                window.pop_front();
                record.remove(&s[lp]);
                lp += 1;
            }
            window.pop_front();
            lp += 1;
        } else {
            record.insert(c);
        }

        if window.len() > max {
            max = window.len();
        }
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = "abcabcbb";
        assert_eq!(3, length_of_longest_substring(s.to_string()));
    }

    #[test]
    fn test_case2() {
        let s = "bbbbb";
        assert_eq!(1, length_of_longest_substring(s.to_string()));
    }

    #[test]
    fn test_case3() {
        let s = "pwwkew";
        assert_eq!(3, length_of_longest_substring(s.to_string()));
    }
}
