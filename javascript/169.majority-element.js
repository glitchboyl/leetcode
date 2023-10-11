/*
 * @lc app=leetcode id=169 lang=javascript
 *
 * [169] Majority Element
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var majorityElement = function (nums) {
  let major = nums[0],
    count = 1;
  for (let i = 1; i < nums.length; i++) {
    if (count === 0) major = nums[i];
    if (major === nums[i]) count++;
    else count--;
  }
  return major;
};
// @lc code=end
