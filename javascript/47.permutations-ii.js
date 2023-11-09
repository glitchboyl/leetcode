/*
 * @lc app=leetcode id=47 lang=javascript
 *
 * [47] Permutations II
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permuteUnique = function (nums) {
  const permutations = [];
  const used = new Array(nums.length).fill(false);
  nums.sort((a, b) => a - b);
  function backtrack(permutation) {
    for (let i = 0; i < nums.length; i++) {
      if (used[i] || (i > 0 && nums[i] === nums[i - 1] && used[i - 1]))
        continue;
      used[i] = true;
      permutation.push(nums[i]);
      if (permutation.length === nums.length)
        permutations.push([...permutation]);
      else backtrack(permutation);
      used[i] = false;
      permutation.pop();
    }
  }
  backtrack([]);
  return permutations;
};
// @lc code=end
