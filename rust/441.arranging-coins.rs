/*
 * @lc app=leetcode id=441 lang=rust
 *
 * [441] Arranging Coins
 */

// @lc code=start
impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        // Solution 1:
        // let (mut n, mut row) = (n, 1);
        // while (n - row >= 0) {
        //     n -= row;
        //     row += 1;
        // }
        // row - 1

        // Solution 2: Math
        ((1.0 + 8.0 * n as f64).sqrt() as i32 - 1) >> 1
    }
}
// @lc code=end
