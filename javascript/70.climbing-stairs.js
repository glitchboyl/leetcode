/*
 * @lc app=leetcode id=70 lang=javascript
 *
 * [70] Climbing Stairs
 */

// @lc code=start

/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function (n) {
  // Solution 1: memo
  // const ways = [0, 1, 2, 3];
  // function climb(n) {
  //   if (typeof ways[n] !== "undefined") return ways[n];
  //   ways[n] = climb(n - 1) + climb(n - 2);
  //   return ways[n];
  // }
  // return climb(n);

  // Solution 2: dp
  // const dp = [1, 2];
  // for (let i = 2; i < n; i++) {
  //   dp.push(dp[i - 2] + dp[i - 1]);
  // }
  // return dp[n - 1];

  // Solution 3:
  let p = 0,
    q = 1;
  for (let i = 1; i < n; i++) [p, q] = [q, p + q];
  return p + q;
};
// @lc code=end
