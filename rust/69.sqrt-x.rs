/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        match x {
            0 | 1 => return x,
            _ => {
                // consraint x <= 2^31 - 1
                let sqrt_limit = 46340;
                let (mut left, mut right) = (1, x / 2);
                if right > sqrt_limit {
                    right = sqrt_limit;
                }
                while left <= right {
                    let mid = (left + right) / 2;
                    let sqrt = mid * mid;
                    if sqrt == x {
                        return mid;
                    } else if sqrt > x {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
                return right;
            }
        }
    }
}
// @lc code=end
