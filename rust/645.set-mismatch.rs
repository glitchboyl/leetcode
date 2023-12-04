/*
 * @lc app=leetcode id=645 lang=rust
 *
 * [645] Set Mismatch
 */

// @lc code=start
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut set = HashSet::with_capacity(nums.len());
        let n = nums.len() as i32;
        let (mut duplicate, mut missing) = (0, 0);
        for num in nums {
            if !set.insert(num) {
                duplicate = num;
            }
        }
        for i in 1..=n {
            if !set.contains(&i) {
                missing = i;
                break;
            }
        }
        vec![duplicate, missing]
    }
}
// @lc code=end
