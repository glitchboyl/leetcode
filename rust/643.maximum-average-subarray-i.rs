/*
 * @lc app=leetcode id=643 lang=rust
 *
 * [643] Maximum Average Subarray I
 */

// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum = nums.iter().take(k).sum::<i32>() as f64;
        let mut max_sum = sum;
        for i in k..nums.len() {
            sum += nums[i] as f64 - nums[i - k] as f64;
            max_sum = max_sum.max(sum);
        }
        max_sum / k as f64
    }
}
// @lc code=end
