/*
 * @lc app=leetcode id=46 lang=rust
 *
 * [46] Permutations
 */

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = vec![];
        let mut used = vec![false; nums.len()];
        fn backtrack_insert(
            nums: &Vec<i32>,
            used: &mut Vec<bool>,
            permutation: &mut Vec<i32>,
            permutations: &mut Vec<Vec<i32>>,
        ) {
            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                permutation.push(nums[i]);
                used[i] = true;
                if permutation.len() == nums.len() {
                    permutations.push(permutation.clone());
                } else {
                    backtrack_insert(nums, used, permutation, permutations);
                }
                used[i] = false;
                permutation.pop();
            }
        }
        backtrack_insert(&nums, &mut used, &mut vec![], &mut permutations);
        permutations
    }
}
// @lc code=end
