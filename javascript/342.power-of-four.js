/*
 * @lc app=leetcode id=342 lang=javascript
 *
 * [342] Power of Four
 */

// @lc code=start
/**
 * @param {number} n
 * @return {boolean}
 */
var isPowerOfFour = function (n) {
  if (n > 0) {
    while (n % 4 === 0) {
      n /= 4;
    }
    return n === 1;
  }
  return false;
};
// @lc code=end
