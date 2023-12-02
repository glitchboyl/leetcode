/*
 * @lc app=leetcode id=80 lang=rust
 *
 * [80] Remove Duplicates from Sorted Array II
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut current = nums[0];
        let mut count = 1;
        let mut s = 1;
        for f in 1..nums.len() {
            if nums[f] != current {
                current = nums[f];
                nums.swap(s, f);
                count = 1;
                s += 1;
            } else if count < 2 {
                nums.swap(s, f);
                count += 1;
                s += 1;
            }
        }
        s as i32
    }
}
// @lc code=end
