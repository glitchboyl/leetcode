/*
 * @lc app=leetcode id=504 lang=rust
 *
 * [504] Base 7
 */

// @lc code=start
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        if num < 0 {
            return format!("-{}", Self::convert_to_base7(num * -1));
        }
        let mut num = num;
        let mut result = vec![];
        while num != 0 {
            result.push(num % 7);
            num /= 7;
        }
        result.iter().rev().map(|n| n.to_string()).collect()
    }
}
// @lc code=end
