/*
 * @lc app=leetcode id=594 lang=rust
 *
 * [594] Longest Harmonious Subsequence
 */

// @lc code=start
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut longest_len = 0;
        for n in nums {
            let i = *map.entry(n).and_modify(|count| *count += 1).or_insert(1);
            if let Some(&v) = map.get(&(n - 1)) {
                longest_len = longest_len.max(v + i);
            }
            if let Some(&v) = map.get(&(n + 1)) {
                longest_len = longest_len.max(v + i);
            }
        }
        longest_len
    }
}
// @lc code=end
