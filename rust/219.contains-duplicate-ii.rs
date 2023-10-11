/*
 * @lc app=leetcode id=219 lang=rust
 *
 * [219] Contains Duplicate II
 */

// @lc code=start
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut index_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            if let Some(index) = index_map.insert(nums[i], i) {
                if (i - index) as i32 <= k {
                    return true;
                }
            }
        }
        return false;
    }
}
// @lc code=end
