/*
 * @lc app=leetcode id=263 lang=rust
 *
 * [263] Ugly Number
 */

// @lc code=start
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        let mut n = n;
        if n > 0 {
            while n % 2 == 0 {
                n /= 2;
            }
            while n % 3 == 0 {
                n /= 3;
            }
            while n % 5 == 0 {
                n /= 5;
            }
            return n == 1;
        }
        return false;
    }
}
// @lc code=end
