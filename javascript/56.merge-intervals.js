/*
 * @lc app=leetcode id=56 lang=javascript
 *
 * [56] Merge Intervals
 */

// @lc code=start
/**
 * @param {number[][]} intervals
 * @return {number[][]}
 */
var merge = function (intervals) {
  if (intervals.length === 1) return intervals;
  const result = [];
  intervals.sort((a, b) => a[0] - b[0]);
  let [start, end] = intervals[0];
  for (let i = 1; i < intervals.length; i++) {
    let [s, e] = intervals[i];
    if (s < start) start = s;
    if (end < s) {
      result.push([start, end]);
      start = s;
      end = e;
    }
    if (e > end) end = e;
  }
  result.push([start, end]);
  return result;
};
// @lc code=end
