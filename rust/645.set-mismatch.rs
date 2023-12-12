/*
 * @lc app=leetcode id=645 lang=rust
 *
 * [645] Set Mismatch
 */

// @lc code=start
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() as i32;
        let mut arr = vec![0; nums.len()];
        let (mut duplicate, mut missing) = (0, n * (n + 1) / 2);
        for num in nums {
            if arr[num as usize - 1] == 1 {
                duplicate = num;
                continue;
            }
            arr[num as usize - 1] = 1;
            missing -= num;
        }
        vec![duplicate, missing]
    }
}
// @lc code=end
