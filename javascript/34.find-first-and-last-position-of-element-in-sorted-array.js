/*
 * @lc app=leetcode id=34 lang=javascript
 *
 * [34] Find First and Last Position of Element in Sorted Array
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var searchRange = function (nums, target) {
  if (nums.length === 0) return [-1, -1];

  const findPosition = (isFirst = true, left = 0) => {
    let right = nums.length - 1;
    let position = -1;
    while (left <= right) {
      const mid = (left + right) >> 1;
      if (nums[mid] === target) {
        position = mid;
        if (isFirst) right = mid - 1;
        else left = mid + 1;
      } else if (nums[mid] > target) right = mid - 1;
      else left = mid + 1;
    }
    return position;
  };
  const left = findPosition();
  return [left, left === -1 ? -1 : findPosition(false, left)];
};
// @lc code=end
