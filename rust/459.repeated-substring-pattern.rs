/*
 * @lc app=leetcode id=459 lang=rust
 *
 * [459] Repeated Substring Pattern
 */

// @lc code=start
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();

        // Solution 1:
        // for i in 1..=len / 2 {
        //     if len % i == 0 && s[..i].repeat(len / i) == s {
        //         return true;
        //     }
        // }
        // false

        // Solution 2:
        s.repeat(2)[1..len * 2 - 1].contains(&s)
    }
}
// @lc code=end
