/*
 * @lc app=leetcode id=73 lang=javascript
 *
 * [73] Set Matrix Zeroes
 */

// @lc code=start
/**
 * @param {number[][]} matrix
 * @return {void} Do not return anything, modify matrix in-place instead.
 */
var setZeroes = function (matrix) {
  let firstRowZero = matrix[0].includes(0),
    firstColumnZero = matrix[0][0] === 0;
  for (let i = 1; i < matrix.length; i++) {
    firstColumnZero = firstColumnZero || matrix[i][0] === 0;
    for (let j = 1; j < matrix[0].length; j++) {
      if (matrix[i][j] === 0) {
        matrix[0][j] = 0;
        matrix[i][0] = 0;
      }
    }
  }
  for (let i = matrix.length - 1; i >= 0; i--) {
    for (let j = 1; j < matrix[0].length; j++) {
      if (
        (i === 0 && firstRowZero) ||
        matrix[i][0] === 0 ||
        matrix[0][j] === 0
      ) {
        matrix[i][j] = 0;
      }
    }
    if (firstColumnZero || (i === 0 && firstRowZero)) matrix[i][0] = 0;
  }
};
// @lc code=end
