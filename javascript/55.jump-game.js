/*
 * @lc app=leetcode id=55 lang=javascript
 *
 * [55] Jump Game
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canJump = function (nums) {
  if (nums.length === 1) return true;
  let farther = 0;
  for (let i = 0; i < nums.length - 1; i++) {
    farther = Math.max(farther, i + nums[i]);
    if (i === farther) return false;
  }
  return true;
};
// @lc code=end
