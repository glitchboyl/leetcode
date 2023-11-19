/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // Solution 1: dp
        // let m = m as usize;
        // let n = n as usize;
        // // let mut dp = vec![vec![1; n]; m];
        // // let mut dp = vec![vec![1; n]; 2];
        // let mut dp = vec![1; n];
        // for i in 1..m {
        //     for j in 1..n {
        //         dp[j] += dp[j - 1];
        //     }
        // }
        // dp[n - 1]

        // Solution 2: discrete math
        let m = m as u64 - 1;
        let n = n as u64 - 1;
        (1..=n).fold(1u64, |acc, x| acc * (m + x) / x) as _
    }
}
// @lc code=end
