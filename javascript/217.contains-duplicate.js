/*
 * @lc app=leetcode id=217 lang=javascript
 *
 * [217] Contains Duplicate
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {boolean}
 */
var containsDuplicate = function (nums) {
  // Solution 1:
  return new Set(nums).size !== nums.length;

  // Solution 2:
  // for (let i = 0; i < nums.length; i++) {
  //   if (nums.indexOf(nums[i], i + 1) !== -1) return true;
  // }
  // return false;

  // Solution 3:
  // const appear = {};
  // for (let i = 0; i < nums.length; i++) {
  //   if (appear[nums[i]]) return true;
  //   appear[nums[i]] = true;
  // }
  // return false;
};
// @lc code=end
