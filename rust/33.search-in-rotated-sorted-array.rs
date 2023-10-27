/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            if nums[l] > target && nums[r] < target {
                break;
            }
            if nums[l] == target {
                return l as i32;
            }
            if nums[r] == target {
                return r as i32;
            }
            let m = (l + r) / 2;
            if nums[m] == target {
                return m as i32;
            }
            if nums[m] >= nums[l] {
                if nums[l] < target && nums[m] > target {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else {
                if nums[r] > target && nums[m] < target {
                    l = m + 1;
                } else {
                    r = m - 1;
                }
            }
        }
        -1
    }
}
// @lc code=end
