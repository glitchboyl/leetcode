/*
 * @lc app=leetcode id=29 lang=rust
 *
 * [29] Divide Two Integers
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        if dividend == divisor {
            return 1;
        }
        if divisor == 1 {
            return dividend;
        }
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let is_negative = (dividend > 0) ^ (divisor > 0);
        let mut dividend = dividend.abs() as u32;
        let divisor = divisor.abs() as u32;
        let mut result = 0;
        while dividend >= divisor {
            let mut b = 0;
            while b < 29 && dividend > (divisor << (b + 1)) {
                b += 1;
            }
            result += 1 << b;
            dividend -= divisor << b;
        }

        if is_negative {
            -result
        } else {
            result
        }
    }
}
// @lc code=end
