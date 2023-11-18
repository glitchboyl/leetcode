/*
 * @lc app=leetcode id=57 lang=rust
 *
 * [57] Insert Interval
 */

// @lc code=start
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        if intervals.len() == 0 || intervals[intervals.len() - 1][1] < new_interval[0] {
            intervals.push(new_interval);
            return intervals;
        }
        let mut new_interval = new_interval;
        let l = intervals.partition_point(|interval| interval[1] < new_interval[0]);
        let r = intervals[l..].partition_point(|interval| interval[0] <= new_interval[1]) + l;
        let slice: Vec<Vec<i32>> = intervals.drain(l..r).collect();
        if !slice.is_empty() {
            new_interval[0] = new_interval[0].min(slice[0][0]);
            new_interval[1] = new_interval[1].max(slice[slice.len() - 1][1]);
        }
        intervals.insert(l, new_interval);
        intervals
    }
}
// @lc code=end
