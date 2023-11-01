/*
 * @lc app=leetcode id=482 lang=rust
 *
 * [482] License Key Formatting
 */

// @lc code=start
impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s = s.replace("-", "");
        let mut key = String::new();
        let d = s.len() as i32 % k;
        for (i, c) in s.chars().enumerate() {
            let i = i as i32;
            if i != 0 && (i == d || (i - d) % k == 0) {
                key.push('-');
            }
            key.push(c.to_ascii_uppercase());
        }
        key
    }
}
// @lc code=end
