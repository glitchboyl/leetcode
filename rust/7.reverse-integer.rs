/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let (mut n, mut result): (i32, i32) = (x.abs(), 0);
        while n != 0 {
            if let Some(mul_r) = result.checked_mul(10) {
                if let Some(add_r) = mul_r.checked_add(n % 10) {
                    result = add_r;
                    n /= 10;
                    continue;
                }
            };
            return 0;
        }
        if x < 0 {
            result *= -1;
        }
        return result;
    }
}
// @lc code=end
