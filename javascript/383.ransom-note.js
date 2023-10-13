/*
 * @lc app=leetcode id=383 lang=javascript
 *
 * [383] Ransom Note
 */

// @lc code=start
/**
 * @param {string} ransomNote
 * @param {string} magazine
 * @return {boolean}
 */
var canConstruct = function (ransomNote, magazine) {
  if (ransomNote.length > magazine.length) return false;

  const chars = new Array(26).fill(0);
  const d = "a".charCodeAt();
  for (const c of ransomNote) {
    chars[c.charCodeAt() - d]++;
  }
  for (const c of magazine) {
    chars[c.charCodeAt() - d]--;
  }
  return !chars.some((n) => n > 0);
};
// @lc code=end
