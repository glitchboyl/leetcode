/*
 * @lc app=leetcode id=35 lang=javascript
 *
 * [35] Search Insert Position
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var searchInsert = function (nums, target) {
  let left = 0,
    right = nums.length - 1;
  if (target <= nums[left]) return left;
  else if (target === nums[right]) return right;
  else if (target > nums[right]) return right + 1;
  while (left < right) {
    const middle = ~~((left + right) / 2);
    if (nums[middle] >= target) right = middle;
    else if (nums[middle] < target) {
      left = middle + 1;
    }
  }
  return left;
};
// @lc code=end
