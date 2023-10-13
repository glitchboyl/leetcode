/*
 * @lc app=leetcode id=3 lang=javascript
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function (s) {
  if (s.length <= 1) return s.length;
  const chars = new Array(128).fill(0);
  let slow = 0,
    fast = 0,
    length = 0;
  for (; fast < s.length; fast++) {
    const c = s[fast].charCodeAt();
    if (chars[c] >= slow) {
      length = Math.max(fast - slow, length);
      slow = chars[c];
    }
    chars[c] = fast + 1;
  }
  return Math.max(fast - slow, length);
};
// @lc code=end
