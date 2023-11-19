/*
 * @lc app=leetcode id=557 lang=rust
 *
 * [557] Reverse Words in a String III
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|cs| cs.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
// @lc code=end
