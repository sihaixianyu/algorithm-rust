pub struct Solution;

impl Solution {
    const ALPHABET_SIZE: usize = 26;

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.chars().collect::<Vec<char>>();
        let p = p.chars().collect::<Vec<char>>();

        let mut res = vec![];
        let (m, n) = (s.len(), p.len());
        let (mut s_cnt, mut p_cnt) = ([0; Self::ALPHABET_SIZE], [0; Self::ALPHABET_SIZE]);

        let chr_to_ord = |x: char| (x as usize) - ('a' as usize);

        for chr in p {
            let ord = chr_to_ord(chr);
            p_cnt[ord] += 1;
        }

        let (mut left, mut right) = (0, 0);

        while right < m {
            let chr = s[right];
            right += 1;
            let ord = chr_to_ord(chr);
            s_cnt[ord] += 1;

            while s_cnt[ord] > p_cnt[ord] {
                let head_chr = s[left];
                let head_ord = chr_to_ord(head_chr);
                s_cnt[head_ord] -= 1;
                left += 1;
            }

            if right - left == n {
                res.push(left as i32);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_0() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let res = Solution::find_anagrams(s, p);
        let exp = vec![0, 6];
        assert_eq!(res, exp);
    }

    #[test]
    fn test_case_1() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        let res = Solution::find_anagrams(s, p);
        let exp = vec![0, 1, 2];
        assert_eq!(res, exp);
    }
}
