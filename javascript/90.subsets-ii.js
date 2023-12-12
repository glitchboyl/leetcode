/*
 * @lc app=leetcode id=90 lang=javascript
 *
 * [90] Subsets II
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsetsWithDup = function (nums) {
  const subsets = [];
  nums.sort((a, b) => a - b);
  function backtrack(i, set) {
    subsets.push([...set]);
    for (let j = i; j < nums.length; j++) {
      if (j > i && nums[j] === nums[j - 1]) continue;
      set.push(nums[j]);
      backtrack(j + 1, set);
      set.pop();
    }
  }
  backtrack(0, []);
  return subsets;
};
// @lc code=end
