/*
 * @lc app=leetcode id=168 lang=rust
 *
 * [168] Excel Sheet Column Title
 */

// @lc code=start
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = String::new();
        let mut i = column_number;
        while i > 0 {
            i -= 1;
            result.insert(0, (i % 26 + 65) as u8 as char);
            i /= 26;
        }
        return result;
    }
}
// @lc code=end
