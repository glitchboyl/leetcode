/*
 * @lc app=leetcode id=415 lang=rust
 *
 * [415] Add Strings
 */

// @lc code=start
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut sum_str = String::new();
        let mut num1 = num1.bytes().rev();
        let mut num2 = num2.bytes().rev();
        let mut carry = 0;
        loop {
            let mut sum = carry;
            match (num1.next(), num2.next()) {
                (Some(n1), Some(n2)) => {
                    sum += n1 - b'0' + n2 - b'0';
                }
                (Some(n), None) | (None, Some(n)) => {
                    sum += n - b'0';
                }
                _ => {
                    if carry == 0 {
                        break;
                    }
                }
            }
            carry = sum / 10;
            sum_str.push((b'0' + sum % 10) as char);
        }

        return sum_str.chars().rev().collect();
    }
}
// @lc code=end
