/*
 * @lc app=leetcode id=66 lang=javascript
 *
 * [66] Plus One
 */

// @lc code=start
/**
 * @param {number[]} digits
 * @return {number[]}
 */
var plusOne = function (digits) {
  let r = (digits.pop() || 0) + 1;
  if (r === 10) {
    r = 0;
    digits = plusOne(digits);
  }
  digits.push(r);
  return digits;
};
// @lc code=end
