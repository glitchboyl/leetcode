/*
 * @lc app=leetcode id=50 lang=rust
 *
 * [50] Pow(x, n)
 */

// @lc code=start
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut result = 1.0;
        if n == 0 {
            return result;
        }
        let mut base = x;
        let mut exp = n as i64;
        if exp < 0 {
            base = 1.0 / base;
            exp = -exp;
        }
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            base *= base;
            exp /= 2;
        }
        result
    }
}
// @lc code=end
