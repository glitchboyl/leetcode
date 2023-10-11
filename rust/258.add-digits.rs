/*
 * @lc app=leetcode id=258 lang=rust
 *
 * [258] Add Digits
 */

// @lc code=start
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        // Solution 1:
        // if num < 10 {
        //     return num;
        // }
        // let (mut n, mut result) = (num, 0);
        // while n > 0 {
        //     result += n % 10;
        //     n /= 10;
        // }
        // return Self::add_digits(result);

        // Solution 2: Digital root (https://en.wikipedia.org/wiki/Digital_root)
        let n = num % 9;
        return match n == 0 && num > 0 {
            true => 9,
            false => n,
        };
    }
}
// @lc code=end
