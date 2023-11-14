/*
 * @lc app=leetcode id=507 lang=javascript
 *
 * [507] Perfect Number
 */

// @lc code=start
/**
 * @param {number} num
 * @return {boolean}
 */
var checkPerfectNumber = function (num) {
  if (num % 2 !== 0) return false;

  let perfectNumber = 0;
  for (let i = 2; i <= ~~Math.sqrt(num); i++) {
    if (num % i === 0) perfectNumber += i + num / i;
  }
  return perfectNumber + 1 === num;
};
// @lc code=end
