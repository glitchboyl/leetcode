/*
 * @lc app=leetcode id=461 lang=javascript
 *
 * [461] Hamming Distance
 */

// @lc code=start
/**
 * @param {number} x
 * @param {number} y
 * @return {number}
 */
var hammingDistance = function (x, y) {
  let i = x ^ y,
    d = 0;
  while (i) {
    d += i & 1;
    i >>= 1;
  }
  return d;
};
// @lc code=end
