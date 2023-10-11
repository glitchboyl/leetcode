/*
 * @lc app=leetcode id=350 lang=javascript
 *
 * [350] Intersection of Two Arrays II
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number[]}
 */
var intersect = function (nums1, nums2) {
  const result = [];
  const nums_counters = {};
  for (const n of nums1) {
    nums_counters[n] = ++nums_counters[n] || 1;
  }
  for (const n of nums2) {
    if (nums_counters[n]) {
      result.push(n);
      nums_counters[n]--;
    }
  }
  return result;
};
// @lc code=end
