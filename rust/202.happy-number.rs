/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 */

// @lc code=start
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        if n == 1 {
            return true;
        }

        fn sum_of_digits_sqr(n: i32) -> i32 {
            let (mut num, mut sum) = (n, 0);
            while num > 0 {
                let d = num % 10;
                sum += d * d;
                num /= 10;
            }
            return sum;
        }
        
        let (mut slow, mut fast) = (n, n);
        loop {
            slow = sum_of_digits_sqr(slow);
            fast = sum_of_digits_sqr(sum_of_digits_sqr(fast));
            if slow == 1 || fast == 1 {
                return true;
            }
            if slow == fast {
                break;
            }
        }
        return false;
    }
}
// @lc code=end
