/*
 * @lc app=leetcode id=14 lang=javascript
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
/**
 * @param {string[]} strs
 * @return {string}
 */
var longestCommonPrefix = function (strs) {
  let prefix = "";

  // Solution 1: two dimensional array like
  if (strs.length === 1) return strs[0];
  let i = 0,
    j = 0;
  while (j < strs.length - 1) {
    if (i >= strs[j].length || strs[j][i] !== strs[j + 1][i]) break;
    if (j === strs.length - 2) {
      prefix += strs[j][i++];
      j = 0;
    } else j++;
  }

  // Solution 2: same as Solution 1 but more concise
  // for (let i = 0; i < strs[0].length; i++) {
  //   if (strs.every((str) => str[i] === strs[0][i])) prefix += strs[0][i];
  //   else break;
  // }

  return prefix;
};
// @lc code=end
