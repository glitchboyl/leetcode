/*
 * @lc app=leetcode id=674 lang=rust
 *
 * [674] Longest Continuous Increasing Subsequence
 */

// @lc code=start
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut s = 0;
        for f in 1..=nums.len() {
            if f != nums.len() && nums[f] > nums[f - 1] {
                continue;
            }
            result = result.max((f - s) as i32);
            s = f;
        }
        result
    }
}
// @lc code=end
