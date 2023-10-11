/*
 * @lc app=leetcode id=219 lang=javascript
 *
 * [219] Contains Duplicate II
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {boolean}
 */
var containsNearbyDuplicate = function (nums, k) {
  // Solution 1:
  // const appear = {};
  // for (let i = 0; i < nums.length; i++) {
  //   const num = nums[i];
  //   if (appear.hasOwnProperty(num) && i - appear[num] <= k) return true;
  //   appear[num] = i;
  // }

  // Solution 2:
  for (let i = 0; i < nums.length; i++) {
    const n = nums.indexOf(nums[i], i + 1);
    if (n != -1 && n - i <= k) return true;
  }

  return false;
};
// @lc code=end
