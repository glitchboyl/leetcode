/*
 * @lc app=leetcode id=507 lang=rust
 *
 * [507] Perfect Number
 */

// @lc code=start
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num % 2 != 0 {
            return false;
        }
        let mut perfect_number = 0;
        for i in 2..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                perfect_number += i + (num / i);
            }
        }
        perfect_number + 1 == num
    }
}
// @lc code=end
