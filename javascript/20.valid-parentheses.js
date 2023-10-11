/*
 * @lc app=leetcode id=20 lang=javascript
 *
 * [20] Valid Parentheses
 */

// @lc code=start
/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function (s) {
  const bracket = {
    "(": ")",
    "[": "]",
    "{": "}",
  };
  const stack = [];
  for (let c of s) {
    if (c === "(" || c === "[" || c === "{") stack.push(c);
    else if (bracket[stack.pop()] != c) return false;
  }
  return stack.length === 0;
};
// @lc code=end
