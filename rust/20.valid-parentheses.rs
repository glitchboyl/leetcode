/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack: Vec<char> = Vec::with_capacity(s.len() / 2);
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                _ => match stack.pop() {
                    Some('(') if c == ')' => (),
                    Some('[') if c == ']' => (),
                    Some('{') if c == '}' => (),
                    _ => return false,
                },
            }
        }
        return stack.is_empty();
    }
}
// @lc code=end
