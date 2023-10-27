/*
 * @lc app=leetcode id=405 lang=javascript
 *
 * [405] Convert a Number to Hexadecimal
 */

// @lc code=start
/**
 * @param {number} num
 * @return {string}
 */
var toHex = function (num) {
  // Solution 1: using built-in method
  // return num.toString(16);

  // Solution 2:
  const d = [
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
  ];
  if (num >= 0 && num < 16) return d[num];
  let hex = "";
  if (num < 0) num >>>= 0; // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators#Unsigned_right_shift
  while (num) {
    const b = num % 16;
    hex = d[b < 0 ? 16 + b : b] + hex;
    num = ~~(num / 16);
  }
  return hex;
};
// @lc code=end
