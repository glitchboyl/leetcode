/*
 * @lc app=leetcode id=482 lang=javascript
 *
 * [482] License Key Formatting
 */

// @lc code=start
/**
 * @param {string} s
 * @param {number} k
 * @return {string}
 */
var licenseKeyFormatting = function (s, k) {
  let key = "";
  s = s.replace(/-/g, "").toUpperCase();
  const d = s.length % k;
  for (let i = 0; i < s.length; i++) {
    if (i > 0 && i % k === d) {
      key += "-";
    }
    key += s[i];
  }
  return key;
};
// @lc code=end
