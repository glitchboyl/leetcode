/*
 * @lc app=leetcode id=448 lang=javascript
 *
 * [448] Find All Numbers Disappeared in an Array
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var findDisappearedNumbers = function (nums) {
  const disappeared = [];
  for (const num of nums) {
    const i = Math.abs(num) - 1;
    if (nums[i] > 0) nums[i] *= -1;
  }
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] > 0) disappeared.push(i + 1);
  }
  return disappeared;
};
// @lc code=end
