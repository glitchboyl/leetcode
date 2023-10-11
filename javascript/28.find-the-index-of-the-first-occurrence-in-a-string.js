/*
 * @lc app=leetcode id=28 lang=javascript
 *
 * [28] Find the Index of the First Occurrence in a String
 */

// @lc code=start
/**
 * @param {string} haystack
 * @param {string} needle
 * @return {number}
 */
var strStr = function (haystack, needle) {
  // Solution 1:
  // const occur = [];
  // for (let i = 0; i < haystack.length; i++) {
  //   if (haystack[i] === needle[0]) {
  //     occur.push(i);
  //   }
  // }
  // for (let i of occur) {
  //   if (i + needle.length - 1 > haystack.length) break;
  //   for (let j = 0; j < needle.length; j++) {
  //     if (haystack[i + j] !== needle[j]) break;
  //     if (j === needle.length - 1) return i;
  //   }
  // }
  // return -1;

  // Solution 2:
  return haystack.indexOf(needle);
};
// @lc code=end
