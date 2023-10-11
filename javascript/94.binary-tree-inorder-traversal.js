/*
 * @lc app=leetcode id=94 lang=javascript
 *
 * [94] Binary Tree Inorder Traversal
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
var inorderTraversal = function (root) {
  const result = [];
  if (root) {
    if (root.left) {
      result.push(...inorderTraversal(root.left));
    }
    result.push(root.val);
    if (root.right) {
      result.push(...inorderTraversal(root.right));
    }
  }
  return result;
};
// @lc code=end
