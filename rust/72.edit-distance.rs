/*
 * @lc app=leetcode id=72 lang=rust
 *
 * [72] Edit Distance
 */

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        if word1 == word2 {
            return 0;
        }
        if word1.is_empty() {
            return word2.len() as i32;
        }
        if word2.is_empty() {
            return word1.len() as i32;
        }
        let n = word2.len();
        let mut dp: Vec<i32> = (0..n as i32 + 1).collect();
        for (i, b1) in word1.bytes().enumerate() {
            let mut d = dp[0];
            dp[0] += 1;
            for (j, b2) in word2.bytes().enumerate() {
                let p = dp[1 + j];
                dp[1 + j] = if b1 == b2 { d } else { 1 + dp[j].min(p).min(d) };
                d = p;
            }
        }
        dp[n]
    }
}
// @lc code=end
