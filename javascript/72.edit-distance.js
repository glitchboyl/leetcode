/*
 * @lc app=leetcode id=72 lang=javascript
 *
 * [72] Edit Distance
 */

// @lc code=start
/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var minDistance = function (word1, word2) {
  const n = word2.length;
  const dp = Array.from({ length: n + 1 }, (_, i) => i);
  for (const c of word1) {
    let d = dp[0];
    dp[0]++;
    for (let i = 0; i < n; i++) {
      const p = dp[i + 1];
      dp[i + 1] = c === word2[i] ? d : 1 + Math.min(d, p, dp[i]);
      d = p;
    }
  }
  return dp[n];
};
// @lc code=end
