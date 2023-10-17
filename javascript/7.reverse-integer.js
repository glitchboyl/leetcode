/*
 * @lc app=leetcode id=7 lang=javascript
 *
 * [7] Reverse Integer
 */

// @lc code=start
/**
 * @param {number} x
 * @return {number}
 */
var reverse = function (x) {
  const MIN_NUM = -(2 ** 31),
    MAX_NUM = 2 ** 31 - 1;
  const sign = x < 0 ? -1 : 1;
  x = Math.abs(x);

  const check = () => {
    if (r * sign > MAX_NUM || r * sign < MIN_NUM) return 0;
    else return 1;
  };

  // Solution 1:
  let r = 0;
  while (x !== 0) {
    r *= 10;
    if (check()) {
      r += x % 10;
      if (!check()) return 0;
    } else return 0;
    x = ~~(x / 10);
  }

  // Solution 2:
  // const r = parseInt(x.toString().split('').reverse().join(''));

  return check() ? r * sign : 0;
};
// @lc code=end
