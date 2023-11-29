/*
 * @lc app=leetcode id=77 lang=javascript
 *
 * [77] Combinations
 */

// @lc code=start
/**
 * @param {number} n
 * @param {number} k
 * @return {number[][]}
 */
var combine = function (n, k) {
  const combinations = [];
  function backtrack(i, combination) {
    if (combination.length === k) {
      combinations.push([...combination]);
      return;
    }
    for (; i <= n; i++) {
      combination.push(i);
      backtrack(i + 1, combination);
      combination.pop();
    }
  }
  backtrack(1, []);
  return combinations;
};
// @lc code=end
