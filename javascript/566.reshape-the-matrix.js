/*
 * @lc app=leetcode id=566 lang=javascript
 *
 * [566] Reshape the Matrix
 */

// @lc code=start
/**
 * @param {number[][]} mat
 * @param {number} r
 * @param {number} c
 * @return {number[][]}
 */
var matrixReshape = function (mat, r, c) {
  const m = mat.length,
    n = mat[0].length;
  if (m * n !== r * c || (m === r && n === c)) return mat;
  return mat.flat().reduce((acc, num, i) => {
    if (i % c === 0) {
      acc.push([]);
    }
    acc[acc.length - 1].push(num);
    return acc;
  }, []);
};
// @lc code=end
