/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut zigzag = String::new();
        let bytes = s.as_bytes();
        for row in 0..num_rows {
            let mut i = row as usize;
            let mut d = true;
            while i < s.len() {
                zigzag.push(bytes[i] as char);
                let mut n = 0;
                if row == 0 || row == num_rows - 1 {
                    n = num_rows - 1;
                } else if d {
                    n = num_rows - row - 1;
                } else {
                    n = row;
                }
                i += (2 * n).max(1) as usize;
                d = !d;
            }
        }
        return zigzag;
    }
}
// @lc code=end
