/*
 * @lc app=leetcode id=89 lang=javascript
 *
 * [89] Gray Code
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number[]}
 */
var grayCode = function (n) {
  const code = [];
  for (let i = 0; i < 1 << n; i++) {
    code.push(i ^ (i >> 1));
  }
  return code;
};
// @lc code=end
