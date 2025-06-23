pub struct Solution;

// 给定一个字符串，只包含 () {} []，检查其是否有效的括号
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for char in s.chars() {
            match char {
                '(' => stack.push(char),
                '{' => stack.push(char),
                '[' => stack.push(char),
                ')' => {
                    if stack.is_empty() || stack.pop().unwrap() != '(' {
                        return false;
                    }
                }
                ']' => {
                    if stack.is_empty() || stack.pop().unwrap() != '[' {
                        return false;
                    }
                }
                '}' => {
                    if stack.is_empty() || stack.pop().unwrap() != '{' {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}
