/*
 * @lc app=leetcode id=374 lang=javascript
 *
 * [374] Guess Number Higher or Lower
 */

// @lc code=start
/**
 * Forward declaration of guess API.
 * @param {number} num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * var guess = function(num) {}
 */

/**
 * @param {number} n
 * @return {number}
 */
var guessNumber = function (n) {
  let min = 1,
    max = n;
  while (min < max) {
    const mid = min + ((max - min) >> 1);
    switch (guess(mid)) {
      case 1:
        min = mid + 1;
        break;
      case -1:
        max = mid - 1;
        break;
      default:
        return mid;
    }
  }
  return min;
};
// @lc code=end
