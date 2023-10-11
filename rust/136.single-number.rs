/*
 * @lc app=leetcode id=136 lang=rust
 *
 * [136] Single Number
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // let mut sorted_nums = nums.clone();
        // sorted_nums.sort();
        // let mut i: usize = 0;
        // while i < (sorted_nums.len() - 1) {
        //     let j: usize = i + 1;
        //     if sorted_nums[i] != sorted_nums[j] {
        //         break;
        //     }
        //     i = j + 1;
        // }
        // return sorted_nums[i];

        // using XOR
        return nums.iter().fold(0, |acc, x| acc ^ x);
    }
}
// @lc code=end
