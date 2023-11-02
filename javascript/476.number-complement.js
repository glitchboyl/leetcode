/*
 * @lc app=leetcode id=476 lang=javascript
 *
 * [476] Number Complement
 */

// @lc code=start
/**
 * @param {number} num
 * @return {number}
 */
var findComplement = function (num) {
  // Solution 1:
  // let complement = "";
  // while (num) {
  //   complement = ((num & 1) ^ 1) + complement;
  //   num >>= 1;
  // }
  // return parseInt(complement, 2);

  // Solution 2:
  return num ^ ((1 << (~~Math.log2(num) + 1)) - 1);
};
// @lc code=end
