/*
 * @lc app=leetcode id=367 lang=rust
 *
 * [367] Valid Perfect Square
 */

// @lc code=start
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        // consraint x <= 2^31 - 1
        let sqrt_limit = 46340;
        let (mut left, mut right) = (1, num / 2);
        if right > sqrt_limit {
            right = sqrt_limit;
        }
        while left < right {
            let mid = left + (right - left) / 2;
            if mid * mid == num {
                return true;
            } else if mid * mid < num {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        return left * left == num;
    }
}
// @lc code=end
