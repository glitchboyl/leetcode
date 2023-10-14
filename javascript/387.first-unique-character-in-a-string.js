/*
 * @lc app=leetcode id=387 lang=javascript
 *
 * [387] First Unique Character in a String
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var firstUniqChar = function (s) {
  const chars = new Array(26).fill(0);
  const d = "a".charCodeAt();
  for (const c of s) chars[c.charCodeAt() - d]++;
  for (let i = 0; i < s.length; i++) {
    if (chars[s[i].charCodeAt() - d] === 1) return i;
  }
  return -1;
};
// @lc code=end
