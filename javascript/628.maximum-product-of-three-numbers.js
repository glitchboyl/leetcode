/*
 * @lc app=leetcode id=628 lang=javascript
 *
 * [628] Maximum Product of Three Numbers
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var maximumProduct = function (nums) {
  nums.sort((a, b) => a - b);
  const n = nums.length - 1;
  return Math.max(
    nums[n] * nums[n - 1] * nums[n - 2],
    nums[n] * nums[0] * nums[1]
  );
};
// @lc code=end
