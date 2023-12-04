/*
 * @lc app=leetcode id=637 lang=javascript
 *
 * [637] Average of Levels in Binary Tree
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
 * @param {TreeNode} root
 * @return {number[]}
 */
var averageOfLevels = function (root) {
  const average = [];
  const stack = [root];
  while (stack.length) {
    const l = stack.length;
    let sum = 0;
    for (let i = 0; i < l; i++) {
      const node = stack.shift();
      sum += node.val;
      if (node.left) stack.push(node.left);
      if (node.right) stack.push(node.right);
    }
    average.push(sum / l);
  }
  return average;
};
// @lc code=end
