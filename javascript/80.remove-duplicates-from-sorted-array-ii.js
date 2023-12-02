/*
 * @lc app=leetcode id=80 lang=javascript
 *
 * [80] Remove Duplicates from Sorted Array II
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var removeDuplicates = function (nums) {
  if (nums.length <= 2) return nums.length;
  let current = nums[0],
    s = 1,
    count = 1;
  for (let f = 1; f < nums.length; f++) {
    if (nums[f] !== current) {
      current = nums[f];
      [nums[s], nums[f]] = [nums[f], nums[s]];
      s++;
      count = 1;
    } else if (count < 2) {
      [nums[s], nums[f]] = [nums[f], nums[s]];
      s++;
      count++;
    }
  }
  return s;
};
// @lc code=end
