/*
 * @lc app=leetcode id=551 lang=rust
 *
 * [551] Student Attendance Record I
 */

// @lc code=start
impl Solution {
    pub fn check_record(s: String) -> bool {
        let (mut late, mut absent) = (0, 0);
        for c in s.bytes() {
            match c {
                b'A' => {
                    late = 0;
                    absent += 1;
                    if absent == 2 {
                        return false;
                    }
                }
                b'L' => {
                    late += 1;
                    if late == 3 {
                        return false;
                    }
                }
                _ => late = 0,
            }
        }
        true
    }
}
// @lc code=end
