/*
 * @lc app=leetcode id=81 lang=javascript
 *
 * [81] Search in Rotated Sorted Array II
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {boolean}
 */
var search = function (nums, target) {
  let left = 0,
    right = nums.length - 1;
  if (right < 2) return nums[left] === target || nums[right] === target;
  while (left <= right) {
    while (left < right && nums[left] === nums[left + 1]) left++;
    while (right > left && nums[right] === nums[right - 1]) right--;
    const mid = (left + right) >> 1;
    if (nums[left] === target || nums[mid] === target || nums[right] === target)
      return true;
    if (mid === left) break;
    if (
      (nums[left] < nums[mid] && target > nums[left] && target < nums[mid]) ||
      (nums[left] > nums[mid] && (target < nums[mid] || target > nums[right]))
    ) {
      left++;
      right = mid - 1;
    } else {
      right--;
      left = mid + 1;
    }
  }
  return false;
};
// @lc code=end
