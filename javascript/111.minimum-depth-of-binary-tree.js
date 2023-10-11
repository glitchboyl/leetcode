/*
 * @lc app=leetcode id=111 lang=javascript
 *
 * [111] Minimum Depth of Binary Tree
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
var minDepth = function (root) {
  if (root === null) return 0;

  // Solution 1: BFS
  let minDepth = 0;
  const levels = [[root]];
  while (levels.length) {
    minDepth++;
    const level = levels.pop();
    const newLevel = [];
    for (let node of level) {
      if (!node.left && !node.right) return minDepth;
      if (node.left) newLevel.push(node.left);
      if (node.right) newLevel.push(node.right);
    }
    if (newLevel.length) levels.push(newLevel);
  }

  // Solution 2: DFS
  // return (
  //   1 +
  //   (root.left === null
  //     ? minDepth(root.right)
  //     : root.right === null
  //     ? minDepth(root.left)
  //     : Math.min(minDepth(root.left), minDepth(root.right)))
  // );
};
// @lc code=end
