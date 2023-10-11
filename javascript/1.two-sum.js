/*
 * @lc app=leetcode id=1 lang=javascript
 *
 * [1] Two Sum
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (nums, target) {
  const d = new Map();
  for (let i = 0; i < nums.length; i++) {
    if (d.has(target - nums[i])) {
      return [d.get(target - nums[i]), i];
    }
    d.set(nums[i], i);
  }
};
// @lc code=end
