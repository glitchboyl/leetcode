/*
 * @lc app=leetcode id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut ordered_nums = nums;
        ordered_nums.sort_unstable();
        let mut d = i32::MAX;
        let len = ordered_nums.len();
        for i in 0..len - 2 {
            if i > 0 && ordered_nums[i] == ordered_nums[i - 1] {
                continue;
            }
            let (mut j, mut k) = (i + 1, len - 1);
            while j < k {
                let _d = target - ordered_nums[i] - ordered_nums[j] - ordered_nums[k];
                if _d == 0 {
                    return target;
                } else if _d > 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
                if _d.abs() < d.abs() {
                    d = _d;
                }
            }
        }
        return target - d;
    }
}
// @lc code=end
