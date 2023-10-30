/*
 * @lc app=leetcode id=31 lang=javascript
 *
 * [31] Next Permutation
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var nextPermutation = function (nums) {
  let l = nums.length - 1,
    k = l;
  if (l === 0) return;

  while (k > 0) {
    const found = nums[k] > nums[k - 1];
    k--;
    if (found) break;
    if (k === 0) {
      nums.sort((a, b) => a - b);
      return;
    }
  }

  while (nums[l] <= nums[k]) l--;

  [nums[k], nums[l]] = [nums[l], nums[k]];

  let p = k + 1,
    q = nums.length - 1;
  while (p < q) {
    [nums[p], nums[q]] = [nums[q], nums[p]];
    p++;
    q--;
  }
};
// @lc code=end
