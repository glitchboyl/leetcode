/*
 * @lc app=leetcode id=448 lang=rust
 *
 * [448] Find All Numbers Disappeared in an Array
 */

// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            let j = nums[i].abs() as usize - 1;
            nums[j] = -nums[j].abs();
        }
        nums.iter()
            .enumerate()
            .filter_map(|(i, num)| if *num > 0 { Some(i as i32 + 1) } else { None })
            .collect()
    }
}
// @lc code=end
