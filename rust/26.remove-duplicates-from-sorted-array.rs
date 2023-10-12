/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // Solution 1:
        // nums.dedup();
        // return nums.len() as i32;

        // Solution 2: two pointers
        let (mut s, mut f): (usize, usize) = (0, 1);
        while f < nums.len() {
            if nums[s] != nums[f] {
                s += 1;
                nums.swap(s, f);
            }
            f += 1;
        }
        return (s + 1) as i32;
    }
}
// @lc code=end
