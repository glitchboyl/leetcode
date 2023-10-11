/*
 * @lc app=leetcode id=58 lang=javascript
 *
 * [58] Length of Last Word
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLastWord = function (s) {
  // Solution 1:
  // const strs = s.trim().split(" ");
  // return strs[strs.length - 1].length;

  // Solution 2:
  let result = 0;
  for (let i = s.length - 1; i >= 0; i--) {
    if (s[i] !== " ") result++;
    else if (result > 0) break;
  }
  return result;
};
// @lc code=end
