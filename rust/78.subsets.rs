/*
 * @lc app=leetcode id=78 lang=rust
 *
 * [78] Subsets
 */

// @lc code=start
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut subsets = Vec::new();
        fn backtrack(
            nums: &Vec<i32>,
            start: usize,
            subset: &mut Vec<i32>,
            subsets: &mut Vec<Vec<i32>>,
        ) {
            subsets.push(subset.clone());
            for i in start..nums.len() {
                subset.push(nums[i]);
                backtrack(nums, i + 1, subset, subsets);
                subset.pop();
            }
        }
        backtrack(&nums, 0, &mut vec![], &mut subsets);
        subsets
    }
}
// @lc code=end
