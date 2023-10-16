/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start();
        let mut i = 0;
        let mut sign = 1;
        let mut chars = s.chars();
        if let Some(c) = chars.next() {
            match c {
                '-' => sign = -1,
                '+' => sign = 1,
                _ => {
                    if c.is_ascii_digit() {
                        i += c.to_digit(10).unwrap() as i32;
                    } else {
                        return i;
                    }
                }
            }
        }
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                if let Some(m) = i.checked_mul(10) {
                    if let Some(a) = m.checked_add(c.to_digit(10).unwrap() as i32) {
                        i = a;
                        continue;
                    }
                }
            } else {
                break;
            }
            if sign == 1 {
                return i32::MAX;
            }
            return i32::MIN;
        }
        return i * sign;
    }
}
// @lc code=end
