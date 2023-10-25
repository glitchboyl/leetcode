/*
 * @lc app=leetcode id=22 lang=javascript
 *
 * [22] Generate Parentheses
 */

// @lc code=start
/**
 * @param {number} n
 * @return {string[]}
 */
var generateParenthesis = function (n) {
  const parentheses = [];
  function backtrackInsert(l, r, s) {
    if (l === n && r === n) return parentheses.push(s);
    if (l < n) {
      backtrackInsert(l + 1, r, s + "(");
    }
    if (r < l && r < n) {
      backtrackInsert(l, r + 1, s + ")");
    }
  }
  backtrackInsert(0, 0, "");
  return parentheses;
};
// @lc code=end
