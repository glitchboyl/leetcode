/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ascii: [usize; 128] = [0; 128];
        let mut length: i32 = 0;
        let mut start: usize = 0;
        for (i, c) in s.chars().enumerate() {
            if ascii[c as usize] >= start {
                length = length.max((i - start) as i32);
                start = ascii[c as usize];
            }
            ascii[c as usize] = i + 1;
        }
        return length.max((s.len() - start) as i32);
    }
}
// @lc code=end
