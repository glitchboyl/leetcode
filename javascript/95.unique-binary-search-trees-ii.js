/*
 * @lc app=leetcode id=95 lang=javascript
 *
 * [95] Unique Binary Search Trees II
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {number} n
 * @return {TreeNode[]}
 */
var generateTrees = function (n) {
  const cache = new Array(n)
    .fill(null)
    .map((_) => new Array(n + 1).fill(null).map((__) => []));
  function generateTreesFrom(start, end) {
    if (start > end) {
      return [null];
    }
    if (cache[start - 1][end].length === 0) {
      for (let i = start; i <= end; i++) {
        const leftTrees = generateTreesFrom(start, i - 1);
        const rightTrees = generateTreesFrom(i + 1, end);
        for (const leftTree of leftTrees) {
          for (const rightTree of rightTrees) {
            cache[start - 1][end].push(new TreeNode(i, leftTree, rightTree));
          }
        }
      }
    }
    return cache[start - 1][end];
  }
  return generateTreesFrom(1, n);
};
// @lc code=end
