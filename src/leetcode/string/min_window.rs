use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Minimum Window Substring
    /// Link: https://leetcode.cn/problems/minimum-window-substring/submissions/
    /// Tags: String, SlidingWindow
    pub fn min_window(s: String, t: String) -> String {
        let mut ans = "".to_owned();
        let mut need = HashMap::new();
        let mut wind = HashMap::new();

        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        for c in t {
            let Some(cnt) = need.get_mut(&c) else {
                need.insert(c, 1);
                wind.insert(c, 0);
                continue;
            };
            *cnt += 1;
        }

        let mut valid = 0;
        let mut left = 0;
        let mut right = 0;

        while right < s.len() {
            let mut c = s[right];
            right += 1;

            if let (Some(need_cnt), Some(wind_cnt)) = (need.get(&c), wind.get_mut(&c)) {
                *wind_cnt += 1;
                if need_cnt == wind_cnt {
                    valid += 1;
                }
            }

            if valid != need.len() {
                continue;
            }

            while valid == need.len() {
                c = s[left];
                left += 1;

                if let (Some(need_cnt), Some(wind_cnt)) = (need.get(&c), wind.get_mut(&c)) {
                    if need_cnt == wind_cnt {
                        valid -= 1;
                    }
                    *wind_cnt -= 1;
                }
            }

            let substr_len = right - left + 1;
            if ans == "".to_string() || substr_len < ans.len() {
                ans = s[left - 1..right].iter().collect();
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();

        let res = Solution::min_window(s, t);
        assert_eq!(res, "BANC".to_string());
    }

    #[test]
    fn test_case2() {
        let s = "a".to_string();
        let t = "aa".to_string();

        let res = Solution::min_window(s, t);
        assert_eq!(res, "".to_string());
    }

    #[test]
    fn test_case3() {
        let s = "a".to_string();
        let t = "a".to_string();

        let res = Solution::min_window(s, t);
        assert_eq!(res, "a".to_string());
    }
}
