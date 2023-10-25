/*
 * @lc app=leetcode id=441 lang=javascript
 *
 * [441] Arranging Coins
 */

// @lc code=start
/**
 * @param {number} n
 * @return {number}
 */
var arrangeCoins = function (n) {
  return ~~(Math.sqrt(1 + 8 * n) - 1) >> 1;
};
// @lc code=end
