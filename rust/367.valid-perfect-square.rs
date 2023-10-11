/*
 * @lc app=leetcode id=367 lang=rust
 *
 * [367] Valid Perfect Square
 */

// @lc code=start
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut left, mut right) = (1, i32::min(x / 2, 46340));
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
