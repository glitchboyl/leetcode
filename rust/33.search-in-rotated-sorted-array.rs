/*
 * @lc app=leetcode id=33 lang=rust
 *
 * [33] Search in Rotated Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        if nums[left] > target && nums[right] < target {
            return -1;
        }
        while left <= right {
            let (l, r) = (nums[left], nums[right]);
            if l == target {
                return left as i32;
            }
            if r == target {
                return right as i32;
            }
            let mid = (left + right) / 2;
            let m = nums[mid];
            if m == target {
                return mid as i32;
            }
            if m >= l {
                if l < target && m > target {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if r > target && m < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}
// @lc code=end
