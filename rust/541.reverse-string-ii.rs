/*
 * @lc app=leetcode id=541 lang=rust
 *
 * [541] Reverse String II
 */

// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        s.chars()
            .collect::<Vec<char>>()
            .chunks_mut(2 * k as usize)
            .map(|cs| {
                if cs.len() > k as usize {
                    cs[0..k as usize].reverse();
                } else {
                    cs.reverse()
                }
                cs.iter().collect::<String>()
            })
            .collect::<String>()
    }
}
// @lc code=end
