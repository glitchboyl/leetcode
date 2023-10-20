/*
 * @lc app=leetcode id=409 lang=javascript
 *
 * [409] Longest Palindrome
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var longestPalindrome = function (s) {
  const chars = {};
  for (const c of s) {
    chars[c] = ++chars[c] || 1;
  }
  let hasOdd = false;
  return (
    Object.values(chars).reduce((sum, n) => {
      hasOdd = hasOdd || n % 2 === 1;
      return sum + ((n >> 1) << 1);
    }, 0) + +hasOdd
  );
};
// @lc code=end
