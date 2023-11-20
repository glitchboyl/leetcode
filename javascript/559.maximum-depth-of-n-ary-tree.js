/*
 * @lc app=leetcode id=559 lang=javascript
 *
 * [559] Maximum Depth of N-ary Tree
 */

// @lc code=start
/**
 * // Definition for a Node.
 * function Node(val,children) {
 *    this.val = val;
 *    this.children = children;
 * };
 */

/**
 * @param {Node|null} root
 * @return {number}
 */
var maxDepth = function (root) {
  let depth = 0;
  function traverse(node, d) {
    if (d > depth) depth = d;
    for (const child of node.children) traverse(child, d + 1);
  }
  if (root) {
    traverse(root, 1);
  }
  return depth;
};
// @lc code=end
