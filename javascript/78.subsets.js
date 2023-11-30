/*
 * @lc app=leetcode id=78 lang=javascript
 *
 * [78] Subsets
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsets = function (nums) {
  const sets = [];
  function backtrack(i, set) {
    sets.push([...set]);
    for (; i < nums.length; i++) {
      set.push(nums[i]);
      backtrack(i + 1, set);
      set.pop();
    }
  }
  backtrack(0, []);
  return sets;
};
// @lc code=end
