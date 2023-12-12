/*
 * @lc app=leetcode id=645 lang=javascript
 *
 * [645] Set Mismatch
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var findErrorNums = function (nums) {
  const n = nums.length;
  const arr = new Array(n).fill(0);
  let duplicate = 0,
    missing = (n * (n + 1)) / 2;
  for (let i = 0; i < n; i++) {
    if (arr[nums[i] - 1]) {
      duplicate = nums[i];
      continue;
    }
    arr[nums[i] - 1] = 1;
    missing -= nums[i];
  }
  return [duplicate, missing];
};
// @lc code=end
