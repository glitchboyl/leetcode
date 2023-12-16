/*
 * @lc app=leetcode id=671 lang=javascript
 *
 * [671] Second Minimum Node In a Binary Tree
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
 * @return {number}
 */
var findSecondMinimumValue = function (root) {
  let min = 2 ** 31;
  let exists = false;
  function dfs(node) {
    if (node.left) {
      if (node.val === node.left.val) {
        dfs(node.left);
      } else if (node.left.val < min) {
        exists = true;
        min = node.left.val;
      }
    }
    if (node.right) {
      if (node.val === node.right.val) {
        dfs(node.right);
      } else if (node.right.val < min) {
        exists = true;
        min = node.right.val;
      }
    }
  }
  dfs(root);
  return exists ? min : -1;
};
// @lc code=end
