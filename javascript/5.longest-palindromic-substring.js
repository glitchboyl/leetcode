/*
 * @lc app=leetcode id=5 lang=javascript
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
/**
 * @param {string} s
 * @return {string}
 */
var longestPalindrome = function (s) {
  let start = 0,
    end = 1;
  function getPalindromeFrom(l, r) {
    while (l >= 0 && r < s.length && s[l] === s[r]) {
      l--;
      r++;
    }
    if (r - l - 1 > end - start) {
      end = r;
      start = l + 1;
    }
  }
  for (let i = 1; i < s.length; i++) {
    getPalindromeFrom(i, i);
    getPalindromeFrom(i - 1, i);
  }
  return s.substring(start, end);
};
// @lc code=end
