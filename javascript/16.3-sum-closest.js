/*
 * @lc app=leetcode id=16 lang=javascript
 *
 * [16] 3Sum Closest
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var threeSumClosest = function (nums, target) {
  nums.sort((a, b) => a - b);
  let d = 2 ** 31 - 1;
  for (let i = 0; i < nums.length - 2; i++) {
    if (i > 0 && nums[i] === nums[i - 1]) continue;
    let j = i + 1,
      k = nums.length - 1;
    while (j < k) {
      const _d = target - nums[i] - nums[j] - nums[k];
      if (_d === 0) return target;
      else if (_d > 0) j += 1;
      else k -= 1;
      if (Math.abs(_d) < Math.abs(d)) d = _d;
    }
  }
  return target - d;
};
// @lc code=end
