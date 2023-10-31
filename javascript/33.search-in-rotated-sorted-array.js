/*
 * @lc app=leetcode id=33 lang=javascript
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function (nums, target) {
  let left = 0,
    right = nums.length - 1;
  if (nums[left] > target && nums[right] < target) return -1;
  while (left <= right) {
    const l = nums[left],
      r = nums[right];
    if (l === target) return left;
    if (r === target) return right;
    const mid = (right - left) >> 1;
    const m = nums[mid];
    if (m === target) return mid;
    if (m >= l) {
      if (l < target && m > target) right = mid - 1;
      else left = mid + 1;
    } else {
      if (r > target && m < target) left = mid + 1;
      else right = mid - 1;
    }
  }
  return -1;
};
// @lc code=end
