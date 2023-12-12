/*
 * @lc app=leetcode id=91 lang=rust
 *
 * [91] Decode Ways
 */

// @lc code=start
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes[0] == b'0' {
            return 0;
        }
        if bytes.len() == 1 {
            return 1;
        }
        let (mut d, mut p) = (1, 1);
        for w in bytes.windows(2) {
            let mut n = 0;
            if w[1] != b'0' {
                n += d;
            }
            if w[0] == b'1' || (w[0] == b'2' && w[1] <= b'6') {
                n += p;
            }
            p = d;
            d = n;
        }
        d
    }
}
// @lc code=end