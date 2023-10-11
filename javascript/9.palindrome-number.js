/*
 * @lc app=leetcode id=9 lang=javascript
 *
 * [9] Palindrome Number
 */

// @lc code=start
/**
 * @param {number} x
 * @return {boolean}
 */
var isPalindrome = function (x) {
  if (x < 10) return x >= 0;

  let r = 0,
    n = x;
  while (n > 0) {
    r = r * 10 + (n % 10);
    n = ~~(n / 10);
  }
  return r === x;
};
// @lc code=end
