/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut chars = vec![0; 26];
        for (a, b) in s.bytes().zip(t.bytes()) {
            chars[(a - b'a') as usize] += 1;
            chars[(b - b'a') as usize] -= 1;
        }
        return !chars.iter().any(|x| *x != 0);
    }
}
// @lc code=end
