/*
 * @lc app=leetcode id=59 lang=javascript
 *
 * [59] Spiral Matrix II
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number[][]}
 */
var generateMatrix = function (n) {
  const matrix = new Array(n).fill(null).map(() => new Array(n).fill(0));
  let top = 0,
    right = n - 1,
    bottom = n - 1,
    left = 0;
  let k = 1;
  while (k <= n ** 2) {
    for (let j = left; j <= right; j++) matrix[top][j] = k++;
    top++;

    for (let i = top; i <= bottom; i++) matrix[i][right] = k++;
    right--;

    for (let j = right; j >= left; j--) matrix[bottom][j] = k++;
    bottom--;

    for (let i = bottom; i >= top; i--) matrix[i][left] = k++;
    left++;
  }
  return matrix;
};
// @lc code=end
