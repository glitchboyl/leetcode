/*
 * @lc app=leetcode id=119 lang=javascript
 *
 * [119] Pascal's Triangle II
 */

// @lc code=start
/**
 * @param {number} rowIndex
 * @return {number[]}
 */
var getRow = function (rowIndex) {
  const row = [1];
  if (rowIndex > 0) {
    const prevRow = getRow(rowIndex - 1);
    for (let i = 1; i < rowIndex; i++) {
      row.push(prevRow[i - 1] + prevRow[i]);
    }
    row.push(1);
  }
  return row;
};
// @lc code=end
