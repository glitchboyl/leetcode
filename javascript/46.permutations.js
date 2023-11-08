/*
 * @lc app=leetcode id=46 lang=javascript
 *
 * [46] Permutations
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permute = function (nums) {
  const permutations = [];
  const used = new Array(nums.length).fill(false);
  function backtrack(permutation) {
    for (let i = 0; i < nums.length; i++) {
      if (used[i]) continue;
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
