/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let map = |c: &char| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        let mut p = '\0';
        let mut result = map(&p);
        for c in s.chars().rev() {
            let value = map(&c);
            result += match map(&p) > value {
                true => -value,
                false => value,
            };
            p = c;
        }
        return result;
    }
}
// @lc code=end
