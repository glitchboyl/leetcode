/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let lowercase_str = s.to_lowercase();
        let palindrome = lowercase_str.chars().filter(|x| x.is_ascii_alphanumeric());

        return palindrome.clone().rev().eq(palindrome);
    }
}
// @lc code=end
