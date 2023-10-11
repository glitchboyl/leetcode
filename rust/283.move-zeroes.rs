/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // Solution 1: fill the zeroes
        // let (mut slow, mut fast, mut zeroes): (usize, usize, usize) = (0, 0, 0);
        // let len = nums.len();
        // while fast < len {
        //     if (nums[fast] != 0) {
        //         nums[slow] = nums[fast];
        //         slow += 1;
        //     } else {
        //         zeroes += 1;
        //     }
        //     fast += 1;
        // }
        // while zeroes > 0 {
        //     nums[len - zeroes] = 0;
        //     zeroes -= 1;
        // }

        // Solution 2:
        let mut j: usize = 0;
        for i in 0..nums.len() {
            if (nums[i] != 0) {
                nums.swap(i, j);
                j += 1;
            }
        }
    }
}
// @lc code=end
