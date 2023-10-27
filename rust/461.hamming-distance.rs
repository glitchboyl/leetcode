/*
 * @lc app=leetcode id=461 lang=rust
 *
 * [461] Hamming Distance
 */

// @lc code=start
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        // Solution 1: using built-in method
        // (x ^ y).count_ones() as i32

        // Solution 2:
        let mut d = 0;
        let mut i = x ^ y;
        while i > 0 {
            d += (i & 1) as i32;
            i >>= 1;
        }
        d
    }
}
// @lc code=end
