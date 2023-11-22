/*
 * @lc app=leetcode id=63 lang=javascript
 *
 * [63] Unique Paths II
 */

// @lc code=start
/**
 * @param {number[][]} obstacleGrid
 * @return {number}
 */
var uniquePathsWithObstacles = function (obstacleGrid) {
  const n = obstacleGrid[0].length;
  const dp = new Array(n).fill(0);
  dp[0] = 1;
  for (const row of obstacleGrid) {
    dp[0] = row[0] === 1 ? 0 : dp[0];
    for (let j = 1; j < n; j++) {
      dp[j] = row[j] === 1 ? 0 : dp[j] + dp[j - 1];
    }
  }
  return dp[n - 1];
};
// @lc code=end
