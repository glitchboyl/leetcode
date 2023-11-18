/*
 * @lc app=leetcode id=57 lang=javascript
 *
 * [57] Insert Interval
 */

// @lc code=start
/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @return {number[][]}
 */
var insert = function (intervals, newInterval) {
  if (
    intervals.length === 0 ||
    intervals[intervals.length - 1][1] < newInterval[0]
  ) {
    intervals.push(newInterval);
  } else {
    let l = Math.max(
      intervals.findIndex((interval) => interval[1] >= newInterval[0]),
      0
    );
    let r = l;
    for (; r < intervals.length; r++) {
      if (intervals[r][0] > newInterval[1]) break;
    }
    let slice = intervals.slice(l, r);
    if (slice.length) {
      newInterval[0] = Math.min(newInterval[0], slice[0][0]);
      newInterval[1] = Math.max(newInterval[1], slice[slice.length - 1][1]);
    }
    intervals.splice(l, slice.length, newInterval);
  }
  return intervals;
};
// @lc code=end
