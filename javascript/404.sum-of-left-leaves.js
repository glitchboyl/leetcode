/*
 * @lc app=leetcode id=404 lang=javascript
 *
 * [404] Sum of Left Leaves
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
var sumOfLeftLeaves = function (root) {
  // Solution 1: recursion
  // let sum = 0;
  // if (root) {
  //   const { left, right } = root;
  //   if (left) {
  //     if (!left.left && !left.right) sum += left.val;
  //     else sum += sumOfLeftLeaves(left);
  //   }
  //   sum += sumOfLeftLeaves(right);
  // }
  // return sum;

  // Solution 2: DFS
  function DFS(node, isLeft = false) {
    if (node) {
      if (isLeft && !node.left && !node.right) sum += node.val;
      else {
        DFS(node.left, true);
        DFS(node.right);
      }
    }
  }
  let sum = 0;
  DFS(root);
  return sum;
};
// @lc code=end
