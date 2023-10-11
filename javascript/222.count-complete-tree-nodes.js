/*
 * @lc app=leetcode id=222 lang=javascript
 *
 * [222] Count Complete Tree Nodes
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
var countNodes = function (root) {
  let lh = 0,
    rh = 0;
  let p = root;
  while (p) {
    lh++;
    p = p.left;
  }
  p = root;
  while (p) {
    rh++;
    p = p.right;
  }
  return lh === rh
    ? Math.pow(2, lh) - 1
    : 1 + countNodes(root.left) + countNodes(root.right);
};
// @lc code=end
