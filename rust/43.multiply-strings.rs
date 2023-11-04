/*
 * @lc app=leetcode id=43 lang=rust
 *
 * [43] Multiply Strings
 */

// @lc code=start
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let zero = String::from("0");
        if num1 == zero || num2 == zero {
            return zero;
        }
        let mut result = vec![0; num1.len() + num2.len()];
        for (i, n1) in num1.bytes().rev().enumerate() {
            let n1 = (n1 - b'0') as i32;
            for (j, n2) in num2.bytes().rev().enumerate() {
                let n2 = (n2 - b'0') as i32;
                let r = n1 * n2 + result[i + j];
                result[i + j] = r % 10;
                result[i + j + 1] += r / 10;
            }
        }
        result
            .into_iter()
            .rev()
            .map(|item| item.to_string())
            .collect::<String>()
            .trim_start_matches('0')
            .to_string()
    }
}
// @lc code=end
