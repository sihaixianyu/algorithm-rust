pub fn is_valid(s: String) -> bool {
    let mut stk = vec![];
    for chr in s.chars() {
        let Some(&top) = stk.last() else {
            stk.push(chr);
            continue;
        };

        if is_match(top, chr) {
            stk.pop();
            continue;
        }

        stk.push(chr);
    }

    stk.is_empty()
}

fn is_match(c1: char, c2: char) -> bool {
    match c1 {
        '(' => c2 == ')',
        '[' => c2 == ']',
        '{' => c2 == '}',
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_valid;

    #[test]
    fn test_case_1() {
        let s = "()".to_string();
        let res = is_valid(s);
        assert!(res)
    }

    #[test]
    fn test_case_2() {
        let s = "()[]{}".to_string();
        let res = is_valid(s);
        assert!(res)
    }

    #[test]
    fn test_case_3() {
        let s = "(]".to_string();
        let res = is_valid(s);
        assert!(!res)
    }
}
