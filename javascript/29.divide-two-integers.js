/*
 * @lc app=leetcode id=29 lang=javascript
 *
 * [29] Divide Two Integers
 */

// @lc code=start
/**
 * @param {number} dividend
 * @param {number} divisor
 * @return {number}
 */
var divide = function (dividend, divisor) {
  if (dividend === divisor) return 1;
  if (dividend === 0 || divisor === 1) return dividend;
  const MIN = -(2 ** 31),
    MAX = 2 ** 31 - 1;
  const isNegative = (dividend > 0) ^ (divisor > 0);
  dividend = Math.abs(dividend);
  divisor = Math.abs(divisor);
  let result = 0;
  while (dividend >= divisor) {
    let b = 0;
    let db = divisor << 1;
    while (b < 29 && divisor < db && dividend > db) {
      b += 1;
      db <<= 1;
    }
    result += 1 << b;
    dividend -= divisor << b;
  }

  return isNegative
    ? result > MAX
      ? MIN
      : -result
    : result > MAX
    ? MAX
    : result;
};
// @lc code=end
