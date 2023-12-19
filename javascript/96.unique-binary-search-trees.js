/*
 * @lc app=leetcode id=96 lang=javascript
 *
 * [96] Unique Binary Search Trees
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var numTrees = function (n) {
  const cache = new Array(n)
    .fill(null)
    .map((_) => new Array(n + 1).fill(null).map((__) => 0));
  function treesNum(start, end) {
    if (start > end) return 1;
    if (cache[start - 1][end] === 0) {
      for (let i = start; i <= end; i++) {
        cache[start - 1][end] += treesNum(start, i - 1) * treesNum(i + 1, end);
      }
    }
    return cache[start - 1][end];
  }
  return treesNum(1, n);
};
// @lc code=end
