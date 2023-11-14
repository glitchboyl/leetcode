/*
 * @lc app=leetcode id=509 lang=rust
 *
 * [509] Fibonacci Number
 */

// @lc code=start
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return n;
        }
        let (mut p, mut q) = (0, 1);
        for _ in 1..n {
            let r = p + q;
            p = q;
            q = r;
        }
        return q;
    }
}
// @lc code=end

