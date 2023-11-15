/*
 * @lc app=leetcode id=54 lang=javascript
 *
 * [54] Spiral Matrix
 */

// @lc code=start
/**
 * @param {number[][]} matrix
 * @return {number[]}
 */
var spiralOrder = function(matrix) {
    let top = 0, right = matrix[0].length, bottom = matrix.length, left = 0;
    const result = [];
    while (1) {
      if (left === right) break;
      for (let j = left; j < right; j++) {
        result.push(matrix[top][j]);
      }
      top++;
      if (top === bottom) break;
      for (let i = top; i < bottom; i++) {
        result.push(matrix[i][right - 1]);
      }
      right--;
      if (left === right) break;
      for (let j = right - 1; j >= left; j--) {
        result.push(matrix[bottom - 1][j]);
      }
      bottom--;
      if (top === bottom) break;
      for (let i = bottom - 1; i >= top; i--) {
        result.push(matrix[i][left]);
      }
      left++;
    }
    return result;
};
// @lc code=end

