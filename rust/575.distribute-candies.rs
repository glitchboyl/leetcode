/*
 * @lc app=leetcode id=575 lang=rust
 *
 * [575] Distribute Candies
 */

// @lc code=start
impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let limit = candy_type.len() / 2;
        let types: HashSet<_> = candy_type.into_iter().collect();
        types.len().min(limit) as i32
    }
}
// @lc code=end
