/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */

// @lc code=start
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combinations = vec![];
        fn backtrack(
            candidates: &Vec<i32>,
            target: i32,
            s: usize,
            combination: &mut Vec<i32>,
            combinations: &mut Vec<Vec<i32>>,
        ) {
            for i in s..candidates.len() {
                if candidates[i] > target {
                    continue;
                }
                combination.push(candidates[i]);
                if candidates[i] == target {
                    combinations.push(combination.clone());
                } else {
                    backtrack(
                        candidates,
                        target - candidates[i],
                        i,
                        combination,
                        combinations,
                    );
                }
                combination.pop();
            }
        }
        backtrack(&candidates, target, 0, &mut vec![], &mut combinations);
        combinations
    }
}
// @lc code=end
