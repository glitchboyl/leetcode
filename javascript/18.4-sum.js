/*
 * @lc app=leetcode id=18 lang=javascript
 *
 * [18] 4Sum
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[][]}
 */
var fourSum = function (nums, target) {
  const quadruplets = [];
  if (nums.length >= 4) {
    nums.sort((a, b) => a - b);
    for (let a = 0; a < nums.length - 3; a++) {
      if (a > 0 && nums[a] === nums[a - 1]) continue;
      const subA = target - nums[a];
      for (let b = a + 1; b < nums.length - 2; b++) {
        if (b > a + 1 && nums[b] === nums[b - 1]) continue;
        const subB = subA - nums[b];
        let c = b + 1,
          d = nums.length - 1;
        while (c < d) {
          const sum = nums[c] + nums[d];
          if (sum === subB) {
            quadruplets.push([nums[a], nums[b], nums[c], nums[d]]);
            c++;
            d--;
            while (nums[c] === nums[c - 1] && c < d) c++;
          } else if (sum < subB) {
            c++;
            while (nums[c] === nums[c - 1] && c < d) c++;
          } else d--;
        }
      }
    }
  }
  return quadruplets;
};
// @lc code=end
