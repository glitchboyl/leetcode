/*
 * @lc app=leetcode id=15 lang=rust
 *
 * [15] 3Sum
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ordered_nums = nums;
        ordered_nums.sort_unstable();
        let len = ordered_nums.len();
        let mut result = Vec::new();
        for i in 0..len - 2 {
            if i > 0 && ordered_nums[i] == ordered_nums[i - 1] {
                continue;
            }
            let (mut j, mut k) = (i + 1, len - 1);
            while j < k {
                let sum = ordered_nums[i] + ordered_nums[j] + ordered_nums[k];
                if sum == 0 {
                    result.push(vec![ordered_nums[i], ordered_nums[j], ordered_nums[k]]);
                    j += 1;
                    k -= 1;
                    while ordered_nums[j] == ordered_nums[j - 1] && j < k {
                        j += 1;
                    }
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        return result;
    }
}
// @lc code=end
