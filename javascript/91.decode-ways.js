/*
 * @lc app=leetcode id=91 lang=javascript
 *
 * [91] Decode Ways
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var numDecodings = function (s) {
  if (s[0] === "0") return 0;
  if (s.length === 1) return 1;

  let d = 1,
    p = 1;
  for (let i = 0; i < s.length - 1; i++) {
    let n = 0;
    if (s[i + 1] !== "0") n += d;
    if (s[i] === "1" || (s[i] === "2" && s[i + 1] <= 6)) n += p;
    p = d;
    d = n;
  }
  return d;
};
// @lc code=end
