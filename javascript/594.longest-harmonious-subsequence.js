/*
 * @lc app=leetcode id=594 lang=javascript
 *
 * [594] Longest Harmonious Subsequence
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var findLHS = function (nums) {
  const map = new Map();
  let lhsLength = 0;
  for (const num of nums) {
    const l = (map.get(num) || 0) + 1;
    map.set(num, l);
    if (map.has(num - 1)) lhsLength = Math.max(lhsLength, map.get(num - 1) + l);
    if (map.has(num + 1)) lhsLength = Math.max(lhsLength, map.get(num + 1) + l);
  }
  return lhsLength;
};
// @lc code=end
