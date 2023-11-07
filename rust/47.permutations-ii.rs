/*
 * @lc app=leetcode id=47 lang=rust
 *
 * [47] Permutations II
 */

// @lc code=start
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut permutations = Vec::new();
        let mut used = vec![false; nums.len()];
        let mut nums = nums;
        nums.sort();
        fn backtrack(
            nums: &Vec<i32>,
            used: &mut Vec<bool>,
            permutation: &mut Vec<i32>,
            permutations: &mut Vec<Vec<i32>>,
        ) {
            for i in 0..nums.len() {
                if used[i] || (i > 0 && nums[i] == nums[i - 1] && used[i - 1]) {
                    continue;
                }
                used[i] = true;
                permutation.push(nums[i]);
                if permutation.len() == nums.len() {
                    permutations.push(permutation.clone());
                } else {
                    backtrack(nums, used, permutation, permutations);
                }
                used[i] = false;
                permutation.pop();
            }
        }
        backtrack(&nums, &mut used, &mut vec![], &mut permutations);
        permutations
    }
}
// @lc code=end
