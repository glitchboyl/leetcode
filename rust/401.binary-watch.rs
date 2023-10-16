/*
 * @lc app=leetcode id=401 lang=rust
 *
 * [401] Binary Watch
 */

// @lc code=start
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;
        let mut times = Vec::new();
        if turned_on < 9 {
            for h in 0..12i32 {
                for m in 0..60i32 {
                    if h.count_ones() + m.count_ones() == turned_on {
                        times.push(format!("{}:{:02}", h, m));
                    }
                }
            }
        }
        return times;
    }
}
// @lc code=end
