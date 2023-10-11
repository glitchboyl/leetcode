/*
 * @lc app=leetcode id=13 lang=javascript
 *
 * [13] Roman to Integer
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var romanToInt = function (s) {
  const symbols = {
    I: 1,
    V: 5,
    X: 10,
    L: 50,
    C: 100,
    D: 500,
    M: 1000,
  };
  let result = 0;
  let i = 0;
  for (i; i < s.length - 1; i++) {
    const p = symbols[s[i]],
      q = symbols[s[i + 1]];
    if (p < q) result -= p;
    else result += p;
  }
  result += symbols[s[i]];
  return result;
};
// @lc code=end
