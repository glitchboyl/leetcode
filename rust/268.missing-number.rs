/*
 * @lc app=leetcode id=268 lang=rust
 *
 * [268] Missing Number
 */

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let l = nums.len() as i32;
        let mut sum = l * (l + 1) / 2;
        for n in nums {
            sum -= n;
        }
        return sum;
    }
}
// @lc code=end
