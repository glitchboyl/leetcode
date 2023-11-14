/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut p, mut result) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            p = nums[i].max(p + nums[i]);
            result = result.max(p);
        }
        result
    }
}
// @lc code=end
