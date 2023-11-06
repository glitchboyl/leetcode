/*
 * @lc app=leetcode id=495 lang=javascript
 *
 * [495] Teemo Attacking
 */

// @lc code=start
/**
 * @param {number[]} timeSeries
 * @param {number} duration
 * @return {number}
 */
var findPoisonedDuration = function (timeSeries, duration) {
  let poisoned = 0;
  let p = 0;
  for (const t of timeSeries) {
    poisoned += duration - (p > t ? p - t : 0);
    p = t + duration;
  }
  return poisoned;
};
// @lc code=end
