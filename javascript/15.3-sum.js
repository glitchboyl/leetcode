/*
 * @lc app=leetcode id=15 lang=javascript
 *
 * [15] 3Sum
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function (nums) {
  const result = [];
  nums.sort((a, b) => a - b);
  for (let i = 0; i < nums.length - 2; i++) {
    if (i > 0 && nums[i] === nums[i - 1]) continue;
    let j = i + 1,
      k = nums.length - 1;
    while (j < k) {
      const sum = nums[i] + nums[j] + nums[k];
      if (sum === 0) {
        result.push([nums[i], nums[j], nums[k]]);
        j++;
        k--;
        while (nums[j] === nums[j - 1] && j < k) j++;
      } else if (sum < 0) j++;
      else k--;
    }
  }
  return result;
};
// @lc code=end
