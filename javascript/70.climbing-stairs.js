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
  const ways = [0, 1, 2, 3];

  function climb(n) {
    if (typeof ways[n] !== "undefined") return ways[n];
    ways[n] = climb(n - 1) + climb(n - 2);
    return ways[n];
  }

  return climb(n);
};
// @lc code=end
