/*
 * @lc app=leetcode id=434 lang=rust
 *
 * [434] Number of Segments in a String
 */

// @lc code=start
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut count = 0;
        let mut ns = false;
        for c in s.bytes() {
            let is_space = c == b' ';
            if is_space && ns {
                count += 1;
            }
            ns = !is_space;
        }
        return count + ns as i32;
    }
}
// @lc code=end
