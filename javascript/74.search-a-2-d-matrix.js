/*
 * @lc app=leetcode id=74 lang=javascript
 *
 * [74] Search a 2D Matrix
 */

// @lc code=start
/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
var searchMatrix = function (matrix, target) {
  const m = matrix.length,
    n = matrix[0].length;
  if (target < matrix[0][0] || target > matrix[m - 1][n - 1]) return false;
  if (m === 1 && n === 1) return matrix[0][0] === target;
  let top = 0,
    bottom = m - 1;
  while (top <= bottom) {
    const mid = (top + bottom) >> 1;
    if (matrix[mid][0] <= target && matrix[mid][n - 1] >= target) {
      let left = 0,
        right = n - 1;
      while (left <= right) {
        const middle = (left + right) >> 1;
        if (
          matrix[mid][left] === target ||
          matrix[mid][middle] === target ||
          matrix[mid][right] === target
        )
          return true;
        if (target > matrix[mid][middle]) {
          right--;
          left = middle + 1;
        } else {
          left++;
          right = middle - 1;
        }
      }
      break;
    } else if (matrix[mid][0] > target) {
      bottom = mid - 1;
    } else {
      top = mid + 1;
    }
  }
  return false;
};
// @lc code=end
