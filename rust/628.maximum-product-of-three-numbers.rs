/*
 * @lc app=leetcode id=628 lang=rust
 *
 * [628] Maximum Product of Three Numbers
 */

// @lc code=start
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();

        (nums[n - 1] * nums[n - 2] * nums[n - 3]).max(nums[0] * nums[1] * nums[n - 1])
    }
}
// @lc code=end
