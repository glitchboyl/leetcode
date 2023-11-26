/*
 * @lc app=leetcode id=75 lang=rust
 *
 * [75] Sort Colors
 */

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut i = l;
        while i <= r {
            match nums[i] {
                0 => {
                    nums.swap(i, l);
                    l += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(i, r);
                    if r < 1 {
                        break;
                    }
                    r -= 1;
                }
                _ => i += 1,
            };
        }
    }
}
// @lc code=end
