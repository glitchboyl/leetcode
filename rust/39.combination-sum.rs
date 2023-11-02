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
            combination: Vec<i32>,
            combinations: &mut Vec<Vec<i32>>,
        ) {
            for i in 0..candidates.len() {
                if candidates[i] > target {
                    continue;
                }
                let mut combination = combination.clone();
                combination.push(candidates[i]);
                if candidates[i] == target {
                    combinations.push(combination);
                } else if candidates[i] < target {
                    backtrack(
                        &candidates[i..].to_vec(),
                        target - candidates[i],
                        combination,
                        combinations,
                    );
                }
            }
        }
        backtrack(&candidates, target, vec![], &mut combinations);
        combinations
    }
}
// @lc code=end
