/*
 * @lc app=leetcode id=561 lang=javascript
 *
 * [561] Array Partition
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var arrayPairSum = function (nums) {
  nums.sort((a, b) => a - b);
  return nums.reduce((acc, n, i) => (acc += i % 2 === 0 ? n : 0), 0);
};
// @lc code=end
