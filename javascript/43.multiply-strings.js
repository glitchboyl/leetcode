/*
 * @lc app=leetcode id=43 lang=javascript
 *
 * [43] Multiply Strings
 */

// @lc code=start
/**
 * @param {string} num1
 * @param {string} num2
 * @return {string}
 */
var multiply = function (num1, num2) {
  const zero = "0";
  if (num1 === zero || num2 === zero) return zero;
  const d = zero.charCodeAt();
  const num1len = num1.length;
  const num2len = num2.length;
  const result = new Array(num1len + num2len).fill(0);
  for (let i = num1len - 1; i >= 0; i--) {
    const n1 = num1[i].charCodeAt() - d;
    for (let j = num2len - 1; j >= 0; j--) {
      const n2 = num2[j].charCodeAt() - d;
      const r = n1 * n2 + result[i + j + 1];
      result[i + j + 1] = r % 10;
      result[i + j] += ~~(r / 10);
    }
  }
  return result.join("").replace(/^0+/, "");
};
// @lc code=end
