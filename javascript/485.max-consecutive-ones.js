/*
 * @lc app=leetcode id=485 lang=javascript
 *
 * [485] Max Consecutive Ones
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var findMaxConsecutiveOnes = function (nums) {
  let count = 0,
    result = 0;
  nums.forEach((n) => {
    if (n === 1) {
      count++;
      result = Math.max(count, result);
    } else count = 0;
  });
  return result;
};
// @lc code=end
