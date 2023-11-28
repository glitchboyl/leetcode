/*
 * @lc app=leetcode id=590 lang=javascript
 *
 * [590] N-ary Tree Postorder Traversal
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
 * @return {number[]}
 */
var postorder = function (root) {
  const nodes = [];
  if (root) {
    const stack = [root];
    while (stack.length) {
      const currentNode = stack.pop();
      nodes.push(currentNode.val);
      stack.push(...currentNode.children);
    }
  }
  return nodes.reverse();
};
// @lc code=end
