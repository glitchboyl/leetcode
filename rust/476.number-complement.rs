/*
 * @lc app=leetcode id=476 lang=rust
 *
 * [476] Number Complement
 */

// @lc code=start
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        num ^ (2i32.pow(32 - num.leading_zeros()) - 1)
    }
}
// @lc code=end
