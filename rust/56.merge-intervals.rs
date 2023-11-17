/*
 * @lc app=leetcode id=56 lang=rust
 *
 * [56] Merge Intervals
 */

// @lc code=start
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut intervals = Vec::new();
        let (mut start, mut end) = (sorted_intervals[0][0], sorted_intervals[0][1]);
        for i in 1..sorted_intervals.len() {
            let (s, e) = (sorted_intervals[i][0], sorted_intervals[i][1]);
            if s < start {
                start = s;
            }
            if end < s {
                intervals.push(vec![start, end]);
                start = s;
            }
            if end < e {
                end = e;
            }
        }
        intervals.push(vec![start, end]);
        intervals
    }
}
// @lc code=end
