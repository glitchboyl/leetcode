/*
 * @lc app=leetcode id=55 lang=rust
 *
 * [55] Jump Game
 */

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut farther = 0;
        for i in 0..nums.len() - 1 {
            farther = farther.max(i + nums[i] as usize);
            if i == farther {
                return false;
            }
        }
        true
    }
}
// @lc code=end

