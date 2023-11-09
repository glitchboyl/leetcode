/*
 * @lc app=leetcode id=501 lang=javascript
 *
 * [501] Find Mode in Binary Search Tree
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
var findMode = function (root) {
  const modes = [];
  let current = 0;
  let current_times = 0;
  let max_times = 0;
  function inorder_traversal(node) {
    if (node) {
      inorder_traversal(node.left);
      current_times = node.val === current ? current_times + 1 : 1;
      current = node.val;
      if (current_times >= max_times) {
        if (current_times > max_times) {
          modes.length = 0;
          max_times = current_times;
        }
        modes.push(node.val);
      }
      inorder_traversal(node.right);
    }
  }
  inorder_traversal(root);
  return modes;
};
// @lc code=end
