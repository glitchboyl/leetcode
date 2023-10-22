/*
 * @lc app=leetcode id=414 lang=javascript
 *
 * [414] Third Maximum Number
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number}
 */
var thirdMax = function (nums) {
  const MIN = -(2 ** 31) - 1;
  let first = MIN,
    second = MIN,
    third = MIN;
  for (const n of nums) {
    if (n === third || n === second || n === first) continue;
    if (n > third) {
      third = n;
      if (n > second) {
        third = second;
        second = n;
        if (n > first) {
          second = first;
          first = n;
        }
      }
    }
  }
  return third === MIN ? first : third;
};
// @lc code=end
