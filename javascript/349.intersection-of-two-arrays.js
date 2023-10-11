/*
 * @lc app=leetcode id=349 lang=javascript
 *
 * [349] Intersection of Two Arrays
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number[]}
 */
var intersection = function (nums1, nums2) {
  const result = [];
  const nums_set = new Set(nums1);
  for (const n of nums2) {
    if (nums_set.has(n)) {
      result.push(n);
      nums_set.delete(n);
    }
  }
  return result;
};
// @lc code=end
