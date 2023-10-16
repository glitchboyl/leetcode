/*
 * @lc app=leetcode id=191 lang=rust
 *
 * [191] Number of 1 Bits
 */

// @lc code=start
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        // Solution 1:
        return n.count_ones() as i32;

        // Solution 2:
        // let (mut result, mut x) = (0, n);
        // for i in 0..32 {
        //     result += (x & 1) as i32;
        //     x >>= 1;
        // }
        // return result;
    }
}
// @lc code=end
