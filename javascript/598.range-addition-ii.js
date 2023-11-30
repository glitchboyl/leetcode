/*
 * @lc app=leetcode id=598 lang=javascript
 *
 * [598] Range Addition II
 */

// @lc code=start
/**
 * @param {number} m
 * @param {number} n
 * @param {number[][]} ops
 * @return {number}
 */
var maxCount = function (m, n, ops) {
  for (const [_m, _n] of ops) {
    m = Math.min(m, _m);
    n = Math.min(n, _n);
  }
  return m * n;
};
// @lc code=end
