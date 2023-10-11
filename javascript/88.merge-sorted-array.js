/*
 * @lc app=leetcode id=88 lang=javascript
 *
 * [88] Merge Sorted Array
 */

// @lc code=start
/**
 * @param {number[]} nums1
 * @param {number} m
 * @param {number[]} nums2
 * @param {number} n
 * @return {void} Do not return anything, modify nums1 in-place instead.
 */
var merge = function (nums1, m, nums2, n) {
  // Solution 1:
  // for (let i = 0; i < n; i++) {
  //   nums1[m + i] = nums2[i];
  // }
  // // can use any sort methods
  // nums1.sort((a, b) => a - b);

  // Solution 2: two pointers
  let i = m + n - 1,
    p = m - 1,
    q = n - 1;
  while (q >= 0) {
    nums1[i--] = p >= 0 && nums1[p] > nums2[q] ? nums1[p--] : nums2[q--];
  }
  
  return nums1;
};
// @lc code=end
