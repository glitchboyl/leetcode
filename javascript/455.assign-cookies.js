/*
 * @lc app=leetcode id=455 lang=javascript
 *
 * [455] Assign Cookies
 */

// @lc code=start
/**
 * @param {number[]} g
 * @param {number[]} s
 * @return {number}
 */
var findContentChildren = function (g, s) {
  let contented = 0;
  if (s.length) {
    g.sort((a, b) => a - b);
    s.sort((a, b) => a - b);
    let i = 0,
      j = 0;
    while (i < g.length && j < s.length) {
      if (s[j++] >= g[i]) {
        contented++;
        i++;
      }
    }
  }
  return contented;
};
// @lc code=end
