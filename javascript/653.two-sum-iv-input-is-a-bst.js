/*
 * @lc app=leetcode id=653 lang=javascript
 *
 * [653] Two Sum IV - Input is a BST
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
 * @param {number} k
 * @return {boolean}
 */
var findTarget = function (root, k) {
  const set = new Set();
  let result = false;
  function inorderTraverse(node) {
    if (result) return;
    if (node.left) inorderTraverse(node.left);
    if (set.has(k - node.val)) return (result = true);
    set.add(node.val);
    if (node.right) inorderTraverse(node.right);
  }
  inorderTraverse(root);
  return result;
};
// @lc code=end
