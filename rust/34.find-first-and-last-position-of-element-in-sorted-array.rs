/*
 * @lc app=leetcode id=34 lang=rust
 *
 * [34] Find First and Last Position of Element in Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        fn binary_search(nums: &Vec<i32>, target: i32, find_first: bool) -> i32 {
            let (mut left, mut right) = (0, nums.len() as i32 - 1);
            let mut position = -1;
            while left <= right {
                let mid = left + (right - left) / 2;
                let num = nums[mid as usize];
                if num == target {
                    position = mid;
                    match find_first {
                        true => right = mid - 1,
                        false => left = mid + 1,
                    }
                } else if num < target {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            position
        }
        vec![
            binary_search(&nums, target, true),
            binary_search(&nums, target, false),
        ]
    }
}
// @lc code=end
