/*
 * @lc app=leetcode id=541 lang=javascript
 *
 * [541] Reverse String II
 */

// @lc code=start
/**
 * @param {string} s
 * @param {number} k
 * @return {string}
 */
var reverseStr = function (s, k) {
  let str = "";
  let chars = s.split('');
  while (chars.length > 0) {
    const slice = chars.splice(0, 2 * k);
    str +=
      slice.length > k
        ? slice.splice(0, k).reverse().join("") + slice.join("")
        : slice.reverse().join("");
  }
  return str;
};
// @lc code=end
