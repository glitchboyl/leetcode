/*
 * @lc app=leetcode id=643 lang=javascript
 *
 * [643] Maximum Average Subarray I
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number}
 */
var findMaxAverage = function (nums, k) {
  let average = nums.slice(0, k).reduce((acc, n) => acc + n, 0);
  let maxAverage = average;
  for (let i = k; i < nums.length; i++) {
    average += nums[i] - nums[i - k];
    if (average > maxAverage) maxAverage = average;
  }
  return maxAverage / k;
};
// @lc code=end
