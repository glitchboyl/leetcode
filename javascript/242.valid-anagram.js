/*
 * @lc app=leetcode id=242 lang=javascript
 *
 * [242] Valid Anagram
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isAnagram = function (s, t) {
  if (s.length !== t.length) return false;

  // Solution 1: inputs only consist of lowercase English letters
  const chars = new Array(26).fill(0);
  const d = "a".charCodeAt();
  for (let i = 0; i < s.length; i++) {
    chars[s.charCodeAt(i) - d]++;
    chars[t.charCodeAt(i) - d]--;
  }
  return chars.findIndex((c) => c !== 0) === -1;

  // Solution 2: inputs contain Unicode characters
  // const chars = new Map();
  // for (let i = 0; i < s.length; i++) {
  //   const u = chars.get(s[i]) || 0;
  //   chars.set(s[i], u + 1);
  //   const v = chars.get(t[i]) || 0;
  //   chars.set(t[i], v - 1);
  // }
  // for (let t of chars.values()) {
  //   if (t !== 0) return false;
  // }
  // return true;
};
// @lc code=end
