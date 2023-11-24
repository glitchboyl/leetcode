/*
 * @lc app=leetcode id=64 lang=javascript
 *
 * [64] Minimum Path Sum
 */

// @lc code=start
/**
 * @param {number[][]} grid
 * @return {number}
 */
var minPathSum = function (grid) {
  const m = grid.length,
    n = grid[0].length;
  const dp = grid[0].reduce((acc, num, i) => {
    acc.push(num + (i === 0 ? 0 : acc[acc.length - 1]));
    return acc;
  }, []);
  for (let i = 1; i < m; i++) {
    dp[0] += grid[i][0];
    for (let j = 1; j < n; j++) {
      dp[j] = grid[i][j] + Math.min(dp[j], dp[j - 1]);
    }
  }
  return dp[n - 1];
};
// @lc code=end
