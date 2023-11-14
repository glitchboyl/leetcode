/*
 * @lc app=leetcode id=509 lang=javascript
 *
 * [509] Fibonacci Number
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var fib = function (n) {
  if (n === 0 || n === 1) return n;
  let p = 0,
    q = 1;
  for (let _ = 1; _ < n; _++) {
    [p, q] = [q, p + q];
  }
  return q;
};
// @lc code=end
