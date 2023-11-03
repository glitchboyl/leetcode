/*
 * @lc app=leetcode id=38 lang=javascript
 *
 * [38] Count and Say
 */

// @lc code=start
/**
 * @param {number} n
 * @return {string}
 */
var countAndSay = function (n) {
  let say = "1";
  while (--n) {
    let count = "";
    let s = 0,
      f = 1;
    for (; f < say.length; f++) {
      if (say[f] !== say[s]) {
        count += f - s + say[s];
        s = f;
      }
    }
    count += f - s + say[s];
    say = count;
  }
  return say;
};
// @lc code=end
