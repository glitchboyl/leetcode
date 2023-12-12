/*
 * @lc app=leetcode id=657 lang=rust
 *
 * [657] Robot Return to Origin
 */

// @lc code=start
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut position = (0, 0);
        for d in moves.chars() {
            match d {
                'U' => position.0 -= 1,
                'R' => position.1 += 1,
                'D' => position.0 += 1,
                _ => position.1 -= 1,
            }
        }
        position == (0, 0)
    }
}
// @lc code=end
