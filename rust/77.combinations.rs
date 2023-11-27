/*
 * @lc app=leetcode id=77 lang=rust
 *
 * [77] Combinations
 */

// @lc code=start
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut combinations = Vec::new();
        fn backtrack(
            i: i32,
            n: i32,
            k: usize,
            combination: &mut Vec<i32>,
            combinations: &mut Vec<Vec<i32>>,
        ) {
            if combination.len() == k {
                combinations.push(combination.clone());
                return;
            }
            for j in i..=n {
                combination.push(j);
                backtrack(j + 1, n, k, combination, combinations);
                combination.pop();
            }
        }
        backtrack(1, n, k as usize, &mut vec![], &mut combinations);
        combinations
    }
}
// @lc code=end
