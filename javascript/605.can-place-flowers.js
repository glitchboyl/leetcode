/*
 * @lc app=leetcode id=605 lang=javascript
 *
 * [605] Can Place Flowers
 */

// @lc code=start
/**
 * @param {number[]} flowerbed
 * @param {number} n
 * @return {boolean}
 */
var canPlaceFlowers = function (flowerbed, n) {
  let flowers = 0;
  for (let i = 0; i < flowerbed.length; i++) {
    if (flowerbed[i] === 1) continue;
    if (
      (i === 0 || flowerbed[i - 1] === 0) &&
      (i === flowerbed.length - 1 || flowerbed[i + 1] === 0)
    ) {
      flowerbed[i] = 1;
      flowers++;
    }
  }
  return flowers >= n;
};
// @lc code=end
