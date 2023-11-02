/*
 * @lc app=leetcode id=485 lang=rust
 *
 * [485] Max Consecutive Ones
 */

// @lc code=start
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut count, mut result) = (0, 0);
        for n in nums {
            match n {
                0 => count = 0,
                1 => count += 1,
                _ => (),
            }
            result = result.max(count);
        }
        result
    }
}
// @lc code=end
