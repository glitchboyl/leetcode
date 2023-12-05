/*
 * @lc app=leetcode id=90 lang=rust
 *
 * [90] Subsets II
 */

// @lc code=start
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsets = Vec::new();
        let mut nums = nums;
        nums.sort_unstable();
        fn backtrack(
            nums: &Vec<i32>,
            i: usize,
            subset: &mut Vec<i32>,
            subsets: &mut Vec<Vec<i32>>,
        ) {
            subsets.push(subset.clone());
            for j in i..nums.len() {
                if j > i && nums[j] == nums[j - 1] {
                    continue;
                }
                subset.push(nums[j]);
                backtrack(nums, j + 1, subset, subsets);
                subset.pop();
            }
        }
        backtrack(&nums, 0, &mut vec![], &mut subsets);
        subsets
    }
}
// @lc code=end
