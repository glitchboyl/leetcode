/*
 * @lc app=leetcode id=459 lang=javascript
 *
 * [459] Repeated Substring Pattern
 */

// @lc code=start
/**
 * @param {string} s
 * @return {boolean}
 */
var repeatedSubstringPattern = function (s) {
  // Solution 1:
  // const l = s.length;
  // for (let i = 1; i <= l >> 1; i++) {
  //   if (l % i === 0 && s.slice(0, i).repeat(l / i) === s) return true;
  // }
  // return false;

  // Solution 2:
  return s.repeat(2).slice(1, -1).includes(s);
};
// @lc code=end
