/*
 * @lc app=leetcode id=171 lang=javascript
 *
 * [171] Excel Sheet Column Number
 */

// @lc code=start
/**
 * @param {string} columnTitle
 * @return {number}
 */
var titleToNumber = function (columnTitle) {
  let columnNumber = 0;
  for (let i = 0; i < columnTitle.length; i++) {
    columnNumber +=
      (columnTitle[i].charCodeAt() - 64) *
      Math.pow(26, columnTitle.length - i - 1);
  }
  return columnNumber;
};
// @lc code=end
