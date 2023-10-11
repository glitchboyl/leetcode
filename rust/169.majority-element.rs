/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut count, mut result) = (1, nums[0]);
        for i in 1..nums.len() {
            if count == 0 {
                result = nums[i];
            }
            match nums[i] == result {
                true => count += 1,
                false => count -= 1,
            }
        }
        return result;
    }
}
// @lc code=end
