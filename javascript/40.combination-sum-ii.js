/*
 * @lc app=leetcode id=40 lang=javascript
 *
 * [40] Combination Sum II
 */

// @lc code=start
/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum2 = function (candidates, target) {
  candidates.sort((a, b) => a - b);
  const combinations = [];
  function backtrack(i, target, combination) {
    for (let j = i; j < candidates.length; j++) {
      if (j > i && candidates[j] === candidates[j - 1]) continue;
      if (candidates[j] > target) return;
      combination.push(candidates[j]);
      if (candidates[j] === target) {
        combinations.push([...combination]);
        combination.pop();
        break;
      }
      backtrack(j + 1, target - candidates[j], combination);
      combination.pop();
    }
  }
  backtrack(0, target, []);
  return combinations;
};
// @lc code=end
