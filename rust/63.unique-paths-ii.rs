/*
 * @lc app=leetcode id=63 lang=rust
 *
 * [63] Unique Paths II
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let n = obstacle_grid[0].len();
        let mut dp = vec![0; n];
        dp[0] = 1;
        for row in &obstacle_grid {
            dp[0] = if row[0] == 1 { 0 } else { dp[0] };
            for j in 1..n {
                dp[j] = if row[j] == 1 { 0 } else { dp[j] + dp[j - 1] };
            }
        }
        dp[n - 1]
    }
}
// @lc code=end
