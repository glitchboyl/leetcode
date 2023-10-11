/*
 * @lc app=leetcode id=268 lang=javascript
 *
 * [268] Missing Number
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var missingNumber = function (nums) {
  const n = nums.length;
  let sum = (n * (n + 1)) >> 1;
  for (const num of nums) {
    sum -= num;
  }
  return sum;
};
// @lc code=end
