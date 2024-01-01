use std::collections::VecDeque;

// 基本计算器II：https://leetcode.cn/problems/basic-calculator-ii/
pub fn calculate(s: String) -> i32 {
    fn helper(u8s: &mut VecDeque<u8>) -> i32 {
        let mut stk = vec![];
        let mut num = 0;
        let mut opt = '+';

        while let Some(c) = u8s.pop_front() {
            let c = c as char;

            if c as char == '(' {
                num = helper(u8s);
            }

            if c.is_alphanumeric() {
                num = 10 * num + (c as i32 - '0' as i32);
            }

            if (!c.is_alphanumeric() && c != ' ') || u8s.len() == 0 {
                match opt {
                    '+' => stk.push(num),
                    '-' => stk.push(-num),
                    '*' => {
                        let top = stk.pop().unwrap();
                        stk.push(top * num);
                    }
                    '/' => {
                        let top = stk.pop().unwrap();
                        stk.push(top / num);
                    }
                    _ => {}
                }

                num = 0;
                opt = c;
            }

            if c == ')' {
                break;
            }
        }

        stk.into_iter().sum()
    }

    let mut u8s = VecDeque::from(s.as_bytes().to_vec());
    let res = helper(&mut u8s);

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let s = "1 + 1";
        let res = calculate(s.to_owned());
        assert_eq!(res, 2);
    }

    #[test]
    fn test_case2() {
        let s = " 2-1 + 2 ";
        let res = calculate(s.to_owned());
        assert_eq!(res, 3);
    }

    #[test]
    fn test_case3() {
        let s = "(1+(4+5+2)-3)+(6+8)";
        let res = calculate(s.to_owned());
        assert_eq!(res, 23);
    }

    #[test]
    fn test_case4() {
        let s = "42";
        let res = calculate(s.to_owned());
        assert_eq!(res, 42);
    }
}
