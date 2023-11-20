/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // Solution 1: sort then compare
        // let mut sorted_nums = nums.to_vec();
        // sorted_nums.sort_unstable();
        // for i in 0..(sorted_nums.len() - 1) {
        //     if sorted_nums[i] == sorted_nums[i + 1] {
        //         return true;
        //     }
        // }

        // Solution 2: HashSet
        use std::collections::HashSet;
        let mut num_set: HashSet<i32> = HashSet::with_capacity(nums.len());
        for i in 0..nums.len() {
            if num_set.contains(&nums[i]) {
                return true;
            }
            num_set.insert(nums[i]);
        }

        return false;
    }
}
// @lc code=end
