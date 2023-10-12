/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */

// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut chars = vec![0; 26];
        for c in ransom_note.bytes() {
            chars[(c - b'a') as usize] += 1;
        }
        for c in magazine.bytes() {
            chars[(c - b'a') as usize] -= 1;
        }
        return !chars.iter().any(|x| *x > 0);
    }
}
// @lc code=end
