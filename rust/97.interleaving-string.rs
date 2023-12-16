/*
 * @lc app=leetcode id=97 lang=rust
 *
 * [97] Interleaving String
 */

// @lc code=start
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = true;
        for i in 1..=s2.len() {
            dp[0][i] = dp[0][i - 1] && s2[i - 1] == s3[i - 1];
        }
        for i in 1..=s1.len() {
            dp[i][0] = dp[i - 1][0] && s1[i - 1] == s3[i - 1];
            for j in 1..=s2.len() {
                dp[i][j] = (dp[i - 1][j] && s1[i - 1] == s3[i + j - 1])
                    || (dp[i][j - 1] && s2[j - 1] == s3[i + j - 1]);
            }
        }
        dp[s1.len()][s2.len()]
    }
}
// @lc code=end
