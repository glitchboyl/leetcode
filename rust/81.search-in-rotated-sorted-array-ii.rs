/*
 * @lc app=leetcode id=81 lang=rust
 *
 * [81] Search in Rotated Sorted Array II
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut left, mut right) = (0, nums.len() - 1);
        if right < 2 {
            return nums[left] == target || nums[right] == target;
        }
        while left <= right {
            while left < right && nums[left] == nums[left + 1] {
                left += 1;
            }
            while right > left && nums[right] == nums[right - 1] {
                right -= 1;
            }
            let mid = (left + right) / 2;
            if nums[left] == target || nums[mid] == target || nums[right] == target {
                return true;
            }
            if mid == left {
                break;
            }
            if nums[left] < nums[mid] {
                if target > nums[left] && target < nums[mid] {
                    left += 1;
                    right = mid - 1;
                } else {
                    right -= 1;
                    left = mid + 1;
                }
            } else {
                if target > nums[mid] && target < nums[right] {
                    right -= 1;
                    left = mid + 1;
                } else {
                    left += 1;
                    right = mid - 1;
                }
            }
        }
        false
    }
}
// @lc code=end
