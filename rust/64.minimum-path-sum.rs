/*
 * @lc app=leetcode id=64 lang=rust
 *
 * [64] Minimum Path Sum
 */

// @lc code=start
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp: Vec<i32> = grid[0]
            .iter()
            .scan(0, |state, &x| {
                *state += x;
                Some(*state)
            })
            .collect();
        for i in 1..m {
            dp[0] += grid[i][0];
            for j in 1..n {
                dp[j] = dp[j].min(dp[j - 1]) + grid[i][j];
            }
        }
        dp[n - 1]
    }
}
// @lc code=end
