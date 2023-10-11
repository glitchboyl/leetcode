/*
 * @lc app=leetcode id=69 lang=javascript
 *
 * [69] Sqrt(x)
 */

// @lc code=start
/**
 * @param {number} x
 * @return {number}
 */
var mySqrt = function (x) {
  // Solution 1:
  // return ~~Math.pow(x, 0.5);

  // Solution 2:
  if (x <= 1) return x;
  // consraint x <= 2^31 - 1
  const sqrt_limit = 46340;
  let left = 1,
    right = x >> 1;
  if (right > sqrt_limit) right = sqrt_limit;
  while (left <= right) {
    const mid = (left + right) >> 1;
    const sqrt = mid * mid;
    if (sqrt === x) return mid;
    else if (sqrt > x) {
      right = mid - 1;
    } else {
      left = mid + 1;
    }
  }
  return right;
};
// @lc code=end
