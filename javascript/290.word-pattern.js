/*
 * @lc app=leetcode id=290 lang=javascript
 *
 * [290] Word Pattern
 */

// @lc code=start
/**
 * @param {string} pattern
 * @param {string} s
 * @return {boolean}
 */
var wordPattern = function (pattern, s) {
  const strs = s.split(" ");
  if (strs.length !== pattern.length) return false;

  const matches = {};
  for (let i = 0; i < strs.length; i++) {
    if (
      !matches.hasOwnProperty(`s_${strs[i]}`) &&
      !matches.hasOwnProperty(`p_${pattern[i]}`)
    )
      matches[`p_${(matches[`s_${strs[i]}`] = pattern[i])}`] = strs[i];
    else if (
      matches[`p_${pattern[i]}`] !== strs[i] ||
      matches[`s_${strs[i]}`] !== pattern[i]
    )
      return false;
  }
  return true;
};
// @lc code=end
