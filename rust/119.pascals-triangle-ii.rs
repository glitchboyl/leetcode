/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity((row_index + 1) as usize);
        result.push(1);
        if row_index > 0 {
            let prev_row = Self::get_row(row_index - 1);
            for i in 1..row_index {
                result.push(prev_row[(i - 1) as usize] + prev_row[i as usize]);
            }
            result.push(1);
        }
        return result;
    }
}
// @lc code=end
