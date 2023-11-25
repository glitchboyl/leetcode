/*
 * @lc app=leetcode id=572 lang=javascript
 *
 * [572] Subtree of Another Tree
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
 * @param {TreeNode} subRoot
 * @return {boolean}
 */
var isSubtree = function (root, subRoot) {
  function dfs(node, subNode, comparing) {
    if (node && subNode) {
      let result = false;
      if (node.val === subNode.val) {
        result =
          dfs(node.left, subNode.left, true) &&
          dfs(node.right, subNode.right, true);
      }
      return comparing
        ? result
        : result ||
            dfs(node.left, subNode, false) ||
            dfs(node.right, subNode, false);
    }
    return node === subNode;
  }
  return dfs(root, subRoot, false);
};
// @lc code=end
