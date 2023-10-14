/*
 * @lc app=leetcode id=392 lang=rust
 *
 * [392] Is Subsequence
 */

// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        if s.len() > t.len() {
            return false;
        }
        let s_bytes = s.as_bytes();
        let mut slow = 0;
        for fast in t.bytes() {
            if s_bytes[slow] == fast {
                slow += 1;
            }
            if slow == s.len() {
                return true;
            }
        }
        return false;
    }
}
// @lc code=end
