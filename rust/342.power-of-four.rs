/*
 * @lc app=leetcode id=342 lang=rust
 *
 * [342] Power of Four
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n > 0 {
            let mut n = n;
            while n % 4 == 0 {
                n /= 4;
            }
            return n == 1;
        }
        return false;
    }
}
// @lc code=end
