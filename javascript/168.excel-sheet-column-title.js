/*
 * @lc app=leetcode id=168 lang=javascript
 *
 * [168] Excel Sheet Column Title
 */

// @lc code=start
/**
 * @param {number} columnNumber
 * @return {string}
 */
var convertToTitle = function (columnNumber) {
  let columnTitle = "";
  while (columnNumber > 0) {
    columnNumber -= 1;
    columnTitle = String.fromCharCode(65 + (columnNumber % 26)) + columnTitle;
    columnNumber = ~~(columnNumber / 26);
  }
  return columnTitle;
};
// @lc code=end
