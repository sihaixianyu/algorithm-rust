use std::cmp::max;

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let n = heights.len();

    let mut lstk = vec![-1]; // a decreasing stack from left side
    let mut rstk = vec![n as i32]; // a decreasing stack from right side
    let mut lb = vec![0; n];
    let mut rb = vec![0; n];

    for (i, &v) in heights.iter().enumerate() {
        while lstk.len() > 1 && v <= heights[*lstk.last().unwrap() as usize] {
            lstk.pop();
        }
        lb[i] = *lstk.last().unwrap();
        lstk.push(i as i32);
    }

    for (i, &v) in heights.iter().enumerate().rev() {
        while rstk.len() > 1 && v <= heights[*rstk.last().unwrap() as usize] {
            rstk.pop();
        }
        rb[i] = *rstk.last().unwrap();
        rstk.push(i as i32);
    }

    let mut ans = 0;
    for (i, &v) in heights.iter().enumerate() {
        ans = max(ans, (rb[i] - lb[i] - 1) * v)
    }

    println!("{:?}\n{:?}", lb, rb);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let heights = vec![2, 1, 5, 6, 2, 3];

        let ans = largest_rectangle_area(heights);
        assert_eq!(ans, 10);
    }

    #[test]
    fn test_case2() {
        let heights = vec![2, 4];

        let ans = largest_rectangle_area(heights);
        assert_eq!(ans, 4);
    }
}
