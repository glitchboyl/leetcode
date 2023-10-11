/*
 * @lc app=leetcode id=326 lang=rust
 *
 * [326] Power of Three
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n > 0 {
            let mut n = n;
            while n % 3 == 0 {
                n /= 3;
            }
            return n == 1;
        }
        return false;
    }
}
// @lc code=end
