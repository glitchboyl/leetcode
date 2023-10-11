/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut ac = a.chars().rev();
        let mut bc = b.chars().rev();
        let mut s = 0;
        loop {
            match (ac.next(), bc.next()) {
                (Some(c), Some(d)) => {
                    s = c.to_digit(2).unwrap() + d.to_digit(2).unwrap() + s;
                }
                (Some(c), None) | (None, Some(c)) => {
                    s = c.to_digit(2).unwrap() + s;
                }
                _ => break,
            }
            result.insert(0, char::from_digit(s % 2, 2).unwrap());
            s = (s >= 2) as u32;
        }
        if s != 0 {
            result.insert(0, char::from_digit(1, 2).unwrap());
        }
        return result;
    }
}
// @lc code=end
