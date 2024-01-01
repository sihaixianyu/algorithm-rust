// 逆波兰表达式求职：https://leetcode.cn/problems/evaluate-reverse-polish-notation/
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stk: Vec<i32> = vec![];

    for token in tokens {
        if let Ok(num) = token.parse::<i32>() {
            stk.push(num);
        } else {
            let num2 = stk.pop().unwrap();
            let num1 = stk.pop().unwrap();

            match token.as_str() {
                "+" => stk.push(num1 + num2),
                "-" => stk.push(num1 - num2),
                "*" => stk.push(num1 * num2),
                "/" => stk.push(num1 / num2),
                _ => {}
            }
        }
    }

    stk.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let tokens = vec!["2", "1", "+", "3", "*"];
        let tokens = tokens.into_iter().map(|x| String::from(x)).collect();

        let res = eval_rpn(tokens);
        assert_eq!(res, 9);
    }
}
