/*
 * @lc app=leetcode id=367 lang=javascript
 *
 * [367] Valid Perfect Square
 */

// @lc code=start
/**
 * @param {number} num
 * @return {boolean}
 */
var isPerfectSquare = function (num) {
  let left = 1,
    right = Math.min(num / 2, Math.pow(2, 31) - 1);
  while (left < right) {
    const mid = left + ((right - left) >> 1);
    if (mid * mid === num) return true;
    else if (mid * mid < num) left = mid + 1;
    else right = mid - 1;
  }
  return left * left === num;
};
// @lc code=end
