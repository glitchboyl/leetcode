/*
 * @lc app=leetcode id=561 lang=rust
 *
 * [561] Array Partition
 */

// @lc code=start
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.iter().step_by(2).sum()
    }
}
// @lc code=end