/*
 * @lc app=leetcode id=598 lang=rust
 *
 * [598] Range Addition II
 */

// @lc code=start
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (mut m, mut n) = (m, n);
        for op in ops {
            m = m.min(op[0]);
            n = n.min(op[1]);
        }
        m * n
    }
}
// @lc code=end
