/*
 * @lc app=leetcode id=39 lang=javascript
 *
 * [39] Combination Sum
 */

// @lc code=start
/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum = function (candidates, target) {
  const combinations = [];
  function backtrack(i, target, combination) {
    for (; i < candidates.length; i++) {
      if (candidates[i] > target) continue;
      combination.push(candidates[i]);
      if (candidates[i] === target) combinations.push([...combination]);
      else backtrack(i, target - candidates[i], combination);
      combination.pop();
    }
  }
  backtrack(0, target, []);
  return combinations;
};
// @lc code=end
