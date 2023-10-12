/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        // Solution 1:
        // nums.retain(|&num| num != val);
        // return nums.len() as i32;

        // Solution 2: two pointers
        let (mut s, mut f) = (0, 0);
        while f < nums.len() {
            if nums[f] != val {
                nums.swap(s, f);
                s += 1;
            }
            f += 1;
        }
        return s as i32;
    }
}
// @lc code=end
