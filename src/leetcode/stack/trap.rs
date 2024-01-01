use std::cmp::min;

// 接雨水：https://leetcode.cn/problems/trapping-rain-water/submissions/
pub fn trap(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut stk = vec![]; // a decreasing monotonous stk

    for (idx, &val) in height.iter().enumerate() {
        println!("nxt -> ({}, {})", idx, val);
        println!("stk -> {:?}", stk);

        while stk.len() > 0 && val > height[stk[stk.len()-1]] as i32 {
            let len = stk.len();

            if len > 1 {
                let w = (idx - stk[len - 2] - 1) as i32;
                let h = min(val, height[stk[len - 2]]) - height[stk[len - 1]];
                ans += w * h;
                println!("add -> ans: {}, w: {}, h:{}", ans, w, h);
            }

            println!("pop -> ({}, {})", len-1, height[stk[len-1]]);
            stk.pop();
        }

        println!("push -> ({}, {})", idx, val);
        stk.push(idx);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];

        let res = trap(height);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_case2() {
        let height = vec![4, 2, 0, 3, 2, 5];

        let res = trap(height);
        assert_eq!(res, 9);
    }
}
