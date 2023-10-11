/*
 * @lc app=leetcode id=205 lang=javascript
 *
 * [205] Isomorphic Strings
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isIsomorphic = function (s, t) {
  if (s === t) return true;

  // Solution 1:
  // const s_chars = {};
  // const t_chars = {};
  // for (let i = 0; i < s.length; i++) {
  //   if (!s_chars.hasOwnProperty(s[i])) {
  //     s_chars[s[i]] = t[i];
  //   } else if (s_chars[s[i]] !== t[i]) {
  //     return false;
  //   }
  //   if (!t_chars.hasOwnProperty(t[i])) {
  //     t_chars[t[i]] = s[i];
  //   } else if (t_chars[t[i]] !== s[i]) {
  //     return false;
  //   }
  // }

  // Solution 2:
  for (let i = 0; i < s.length; i++) {
    if (s.indexOf(s[i], i + 1) !== t.indexOf(t[i], i + 1)) return false;
  }

  return true;
};
// @lc code=end
