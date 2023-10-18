/*
 * @lc app=leetcode id=11 lang=javascript
 *
 * [11] Container With Most Water
 */

// @lc code=start
/**
 * @param {number[]} height
 * @return {number}
 */
var maxArea = function (height) {
  let mostContainer = 0;
  let l = 0,
    r = height.length - 1;
  while (l < r) {
    const w = r - l;
    const h = height[l] < height[r] ? height[l++] : height[r--];
    mostContainer = Math.max(w * h, mostContainer);
  }
  return mostContainer;
};
// @lc code=end
