/*
 * @lc app=leetcode id=389 lang=javascript
 *
 * [389] Find the Difference
 */

// @lc code=start
/**
 * @param {string} s
 * @param {string} t
 * @return {character}
 */
var findTheDifference = function (s, t) {
  // Solution 1:
  // const chars = new Array(26).fill(0);
  // const d = "a".charCodeAt();
  // for (const c of t) chars[c.charCodeAt() - d]++;
  // for (const c of s) chars[c.charCodeAt() - d]--;
  // return String.fromCharCode(chars.findIndex((c) => c === 1) + d);

  // Solution 2: using XOR
  let diff = 0;
  for (const c of s) diff ^= c.charCodeAt();
  for (const c of t) diff ^= c.charCodeAt();
  return String.fromCharCode(diff);
};
// @lc code=end
