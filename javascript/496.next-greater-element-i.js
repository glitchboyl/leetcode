/*
 * @lc app=leetcode id=496 lang=javascript
 *
 * [496] Next Greater Element I
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number[]}
 */
var nextGreaterElement = function (nums1, nums2) {
  const greater = {
    [nums2[nums2.length - 1]]: -1,
  };
  for (let i = nums2.length - 2; i >= 0; i--) {
    let n = nums2[i + 1];
    if (nums2[i] > n) {
      while (n !== -1 && n < nums2[i]) n = greater[n];
    }
    greater[nums2[i]] = n;
  }
  return nums1.map((n) => greater[n]);
};
// @lc code=end
