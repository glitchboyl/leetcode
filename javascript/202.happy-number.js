/*
 * @lc app=leetcode id=202 lang=javascript
 *
 * [202] Happy Number
 */

// @lc code=start
/**
 * @param {number} n
 * @return {boolean}
 */
var isHappy = function (n) {
  function process(num) {
    let result = 0;
    while (num) {
      result += (num % 10) * (num % 10);
      num = ~~(num / 10);
    }
    return result;
  }
  let slow = n,
    fast = process(n);
  while (slow !== 1 && fast !== 1) {
    if (slow === fast) return false;
    slow = process(slow);
    fast = process(process(fast));
  }
  return true;
};
// @lc code=end
