/*
 * @lc app=leetcode id=495 lang=rust
 *
 * [495] Teemo Attacking
 */

// @lc code=start
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        time_series
            .iter()
            .fold((0, 0), |acc, t| {
                (
                    acc.0 + duration - if acc.1 > *t { acc.1 - *t } else { 0 },
                    *t + duration,
                )
            })
            .0
    }
}
// @lc code=end
