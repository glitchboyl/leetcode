/*
 * @lc app=leetcode id=40 lang=rust
 *
 * [40] Combination Sum II
 */

// @lc code=start
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combinations = vec![];
        let mut candidates = candidates;
        candidates.sort_unstable();
        fn backtrack(
            candidates: &Vec<i32>,
            target: i32,
            s: usize,
            combination: &mut Vec<i32>,
            combinations: &mut Vec<Vec<i32>>,
        ) {
            for i in s..candidates.len() {
                if i > s && candidates[i] == candidates[i - 1] {
                    continue;
                }
                if candidates[i] > target {
                    return;
                }
                combination.push(candidates[i]);
                if candidates[i] == target {
                    combinations.push(combination.clone());
                    combination.pop();
                    break;
                }
                backtrack(
                    candidates,
                    target - candidates[i],
                    i + 1,
                    combination,
                    combinations,
                );
                combination.pop();
            }
        }
        backtrack(&candidates, target, 0, &mut vec![], &mut combinations);
        combinations
    }
}
// @lc code=end
