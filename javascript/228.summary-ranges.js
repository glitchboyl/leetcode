/*
 * @lc app=leetcode id=228 lang=javascript
 *
 * [228] Summary Ranges
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {string[]}
 */
var summaryRanges = function (nums) {
  const ranges = [];
  let slow = 0;
  for (let fast = 1; fast <= nums.length; fast++) {
    if (fast === nums.length || nums[fast - 1] + 1 !== nums[fast]) {
      let range = `${nums[slow]}`;
      if (slow !== fast - 1) {
        range += `->${nums[fast - 1]}`;
      }
      ranges.push(range);
      slow = fast;
    }
  }
  return ranges;
};
// @lc code=end
