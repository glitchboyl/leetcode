/*
 * @lc app=leetcode id=504 lang=javascript
 *
 * [504] Base 7
 */

// @lc code=start
/**
 * @param {number} num
 * @return {string}
 */
var convertToBase7 = function (num) {
  if (num === 0) return "0";

  // Solution 1:
  // return num.toString(7);

  // Solution 2:
  let result = "";
  let sign = "";
  if (num < 0) {
    sign = "-";
    num = -num;
  }
  while (num) {
    result = (num % 7) + result;
    num = ~~(num / 7);
  }
  return sign + result;
};
// @lc code=end
