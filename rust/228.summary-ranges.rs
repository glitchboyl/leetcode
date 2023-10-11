/*
 * @lc app=leetcode id=228 lang=rust
 *
 * [228] Summary Ranges
 */

// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut slow = 0;
        for fast in 1..=nums.len() {
            if fast == nums.len() || nums[fast] - nums[fast - 1] != 1 {
                let mut range = String::from(nums[slow].to_string());
                if fast - 1 != slow {
                    range.push_str(format!("->{}", nums[fast - 1]).as_ref());
                }
                result.push(range);
                slow = fast;
            }
        }
        return result;
    }
}
// @lc code=end
