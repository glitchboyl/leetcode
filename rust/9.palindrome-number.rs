/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 10 {
            return x >= 0;
        }
        
        // Solution 1: convert to string
        // let s = x.to_string();
        // let n = s.len();
        // for i in 0..&n / 2 {
        //     if s.chars().nth(i).unwrap() != s.chars().nth(n - 1 - i).unwrap() {
        //         return false;
        //     }
        // }
        // return true;

        // Solution 2:
        let (mut r, mut n) = (0, x);
        while n > 0 {
            r = r * 10 + n % 10;
            n /= 10;
        }
        return r == x;
    }
}
// @lc code=end
