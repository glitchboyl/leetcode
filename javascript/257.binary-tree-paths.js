/*
 * @lc app=leetcode id=257 lang=javascript
 *
 * [257] Binary Tree Paths
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
 * @return {string[]}
 */
var binaryTreePaths = function (root) {
  const paths = [];
  function dfs(node, path = "") {
    if (node) {
      const p = path + node.val;
      if (!node.left && !node.right) paths.push(p);
      else {
        dfs(node.left, p + "->");
        dfs(node.right, p + "->");
      }
    }
  }
  dfs(root);
  return paths;
};
// @lc code=end
