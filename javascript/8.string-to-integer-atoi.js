/*
 * @lc app=leetcode id=8 lang=javascript
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
var myAtoi = function (s) {
  const MIN_NUM = -(2 ** 31),
    MAX_NUM = 2 ** 31 - 1;
  let r = "0";
  let i = 0;
  while (i < s.length && s[i] === " ") i++;
  if (s[i] === "-") {
    r = "-" + r;
    i++;
  } else if (s[i] === "+") i++;
  while (i < s.length && s[i] !== " " && !isNaN(s[i])) {
    r += s[i++];
    if (r > MAX_NUM) return MAX_NUM;
    else if (r < MIN_NUM) return MIN_NUM;
  }
  return r;
};
// @lc code=end
