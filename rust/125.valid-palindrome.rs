/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase());

        // Solution 1:
        return s.clone().rev().eq(s);

        // Solution 2: two pointers
        // loop {
        //     match (s.next(), s.next_back()) {
        //         (Some(a), Some(b)) => {
        //             if a != b {
        //                 return false;
        //             }
        //         }
        //         _ => break,
        //     }
        // }
        // return true;
    }
}
// @lc code=end
