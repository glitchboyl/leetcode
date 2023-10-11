/*
 * @lc app=leetcode id=338 lang=rust
 *
 * [338] Counting Bits
 */

// @lc code=start
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![0];
        let n = n as usize;
        for i in 1..n + 1 {
            ans.push(ans[i & (i - 1)] + 1);
        }
        return ans;
    }
}
// @lc code=end
