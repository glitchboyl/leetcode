/*
 * @lc app=leetcode id=190 lang=rust
 *
 * [190] Reverse Bits
 */

// @lc code=start
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        // Solution 1:
        // let (mut result, mut n) = (0u32, x);
        // for i in 0..32 {
        //     result = (result << 1) | (n & 1);
        //     n >>= 1;
        // }
        // return result;

        // Solution 2:
        return x.reverse_bits();
    }
}
// @lc code=end
