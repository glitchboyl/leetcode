/*
 * @lc app=leetcode id=191 lang=javascript
 *
 * [191] Number of 1 Bits
 */

// @lc code=start
/**
 * @param {number} n - a positive integer
 * @return {number}
 */
var hammingWeight = function (n) {
  let w = 0;
  while (n) {
    if (n & 1) w++;
    n >>>= 1;
  }
  return w;
};
// @lc code=end
