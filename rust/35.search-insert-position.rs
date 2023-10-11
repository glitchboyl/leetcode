/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        return nums.binary_search(&target).unwrap_or_else(|x| x) as i32;

        // let (mut left, mut right) = (0, nums.len());
        // while left < right {
        //     let mid = (left + right) / 2;
        //     if nums[mid] < target {
        //         left = mid + 1;
        //     } else {
        //         right = mid;
        //     }
        // }
        // return right as i32;
    }
}
// @lc code=end
