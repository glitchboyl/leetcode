/*
 * @lc app=leetcode id=387 lang=rust
 *
 * [387] First Unique Character in a String
 */

// @lc code=start
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut chars = vec![0; 26];
        for c in s.bytes() {
            chars[(c - b'a') as usize] += 1;
        }
        if let Some(u) = s.bytes().position(|c| chars[(c - b'a') as usize] == 1) {
            return u as i32;
        }
        return -1;
    }
}
// @lc code=end
