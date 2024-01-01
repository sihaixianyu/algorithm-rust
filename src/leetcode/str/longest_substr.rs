use std::collections::HashSet;

// 无重复字符的最长子串：https://leetcode.cn/problems/longest-substring-without-repeating-characters/submissions/
pub fn length_of_longest_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut wind = HashSet::new();
    let mut res = 0;
    let (mut left, mut right) = (0 as usize, 0 as usize);

    while right < bytes.len() {
        let c = bytes[right] as char;
        right += 1;

        if !wind.contains(&c) {
            if right - left > res {
                res = right - left;
            }
        } else {
            while wind.contains(&c) {
                let d = bytes[left] as char;
                wind.remove(&d);
                left += 1;
            }
        }

        wind.insert(c);
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = "abcabcbb".to_string();

        let res = length_of_longest_substring(s);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_case2() {
        let s = "pwwkew".to_string();

        let res = length_of_longest_substring(s);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_case3() {
        let s = "bbbbb".to_string();

        let res = length_of_longest_substring(s);
        assert_eq!(res, 1);
    }
}
