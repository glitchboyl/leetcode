/*
 * @lc app=leetcode id=405 lang=rust
 *
 * [405] Convert a Number to Hexadecimal
 */

// @lc code=start
impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut num = num as u32;
        if num == 0 {
            return "0".to_string();
        }
        let d = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];
        let mut hex = String::new();
        while num != 0 {
            hex = format!("{}{}", d[(num % 16) as usize], hex);
            num /= 16;
        }
        return hex;
    }
}
// @lc code=end
