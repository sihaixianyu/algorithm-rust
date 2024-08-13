use std::collections::HashMap;

// 最小覆盖子串：https://leetcode.cn/problems/minimum-window-substring/submissions/
pub fn min_window(s: String, t: String) -> String {
    let mut need = HashMap::new();
    let mut wind = HashMap::new();

    for c in t.chars() {
        if need.contains_key(&c) {
            *need.get_mut(&c).unwrap() += 1;
        } else {
            need.insert(c, 1);
            wind.insert(c, 0);
        }
    }

    let bytes = s.as_bytes();
    let mut valid = 0;
    let (mut left, mut right) = (0 as usize, 0 as usize);
    let (mut start, mut len) = (0 as usize, usize::MAX);

    while right < bytes.len() {
        let mut c = bytes[right] as char;
        right += 1;

        if need.contains_key(&c) {
            *wind.get_mut(&c).unwrap() += 1;
            if wind.get(&c) == need.get(&c) {
                valid += 1;
            }
        }

        // Shrink the window
        while valid == need.len() {
            if right - left < len {
                start = left;
                len = right - left;
            }

            c = bytes[left] as char;
            if need.contains_key(&c) {
                if need.get(&c) == wind.get(&c) {
                    valid -= 1;
                }
                *wind.get_mut(&c).unwrap() -= 1;
            }

            left += 1;
        }
    }

    if len == usize::MAX {
        return "".to_string();
    }

    s[start..start + len].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();

        let res = min_window(s, t);
        assert_eq!(res, "BANC".to_string());
    }

    #[test]
    fn test_case2() {
        let s = "a".to_string();
        let t = "aa".to_string();

        let res = min_window(s, t);
        assert_eq!(res, "".to_string());
    }

    #[test]
    fn test_case3() {
        let s = "a".to_string();
        let t = "a".to_string();

        let res = min_window(s, t);
        assert_eq!(res, "a".to_string());
    }
}
