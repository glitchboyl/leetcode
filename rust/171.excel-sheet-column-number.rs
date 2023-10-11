/*
 * @lc app=leetcode id=171 lang=rust
 *
 * [171] Excel Sheet Column Number
 */

// @lc code=start
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let (mut n, mut column_number) = (0, 0);
        for i in column_title.chars().rev() {
            column_number += ((i as i32) - 64) * 26_i32.pow(n);
            n += 1;
        }
        return column_number;
    }
}
// @lc code=end
